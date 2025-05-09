#![feature(stdarch_arm_neon)]
use core::arch::aarch64::*;




// #[target_feature(enable = "neon")]
// #[cfg(target_arch = "aarch64")]
// pub unsafe fn barrett_reduce_neon(input: &[i16]) -> Vec<i16> {
//     let len = input.len();
//     let mut result = vec![0i16; len];
    
//     // 处理8个元素一组的部分
//     let chunks = len / 8;
    
//     let v_q = vdupq_n_s16(MLKEM_Q);
//     let v_v = vdupq_n_s16(MLKEM_V);
    
//     for i in 0..chunks {
//         // 加载8个i16值到NEON寄存器
//         let mut a = vld1q_s16(&input[i * 8]);
        
//         // 计算 t = ⌊a·v/2¹⁶⌋ (使用NEON指令)
//         // 由于NEON没有直接的16位乘法然后右移16位的操作，
//         // 我们需要扩展到32位，然后进行乘法和移位
        
//         // 将低4个i16值扩展为i32
//         let a_low = vmovl_s16(vget_low_s16(a));
//         // 将高4个i16值扩展为i32
//         let a_high = vmovl_s16(vget_high_s16(a));
        
//         // 将v_v的低4个i16值扩展为i32
//         let v_v_low = vmovl_s16(vget_low_s16(v_v));
//         // 将v_v的高4个i16值扩展为i32
//         let v_v_high = vmovl_s16(vget_high_s16(v_v));
        
//         // 执行乘法操作
//         let mul_low = vmulq_s32(a_low, v_v_low);
//         let mul_high = vmulq_s32(a_high, v_v_high);
        
//         // 右移16位
//         let t_low = vshrq_n_s32(mul_low, 16);
//         let t_high = vshrq_n_s32(mul_high, 16);
        
//         // 将结果转回i16
//         let t = vcombine_s16(vmovn_s32(t_low), vmovn_s32(t_high));
        
//         // 计算 t * q
//         // 同样，需要分开处理低4位和高4位
//         let t_low_s32 = vmovl_s16(vget_low_s16(t));
//         let t_high_s32 = vmovl_s16(vget_high_s16(t));
        
//         let q_low_s32 = vmovl_s16(vget_low_s16(v_q));
//         let q_high_s32 = vmovl_s16(vget_high_s16(v_q));
        
//         let tq_low = vmulq_s32(t_low_s32, q_low_s32);
//         let tq_high = vmulq_s32(t_high_s32, q_high_s32);
        
//         // 将结果转回i16
//         let tq = vcombine_s16(vmovn_s32(tq_low), vmovn_s32(tq_high));
        
//         // 计算最终结果 r = a - t*q
//         let mut r = vsubq_s16(a, tq);
        
//         // 处理边界情况
//         // 如果r >= q，则r -= q
//         let mask_geq_q = vcgeq_s16(r, v_q);
//         r = vsubq_s16(r, vandq_s16(mask_geq_q, v_q));
        
//         // 如果r < 0，则r += q
//         let mask_lt_0 = vcltq_s16(r, vdupq_n_s16(0));
//         r = vaddq_s16(r, vandq_s16(mask_lt_0, v_q));
        
//         // 存储结果
//         vst1q_s16(&mut result[i * 8], r);
//     }
    
//     // 处理剩余的元素
//     for i in chunks * 8..len {
//         result[i] = barrett_reduce_scalar(input[i]);
//     }
    
//     result
// }

// /// 包装函数，检查CPU特性并选择适当的实现
// pub fn barrett_reduce(input: &[i16]) -> Vec<i16> {
//     #[cfg(target_arch = "aarch64")]
//     {
//         if is_aarch64_feature_detected!("neon") {
//             unsafe { return barrett_reduce_neon(input); }
//         }
//     }
    
//     // 回退到标量实现
//     input.iter().map(|&x| barrett_reduce_scalar(x)).collect()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     #[test]
//     fn test_barrett_reduce_scalar() {
//         // 测试用例
//         let cases = [
//             (0, 0),
//             (MLKEM_Q, 0),
//             (MLKEM_Q + 1, 1),
//             (5000, 5000 - MLKEM_Q),
//             (-1, MLKEM_Q - 1),
//             (-MLKEM_Q, 0),
//         ];
        
//         for (input, expected) in cases.iter() {
//             assert_eq!(barrett_reduce_scalar(*input), *expected);
//         }
//     }
    
//     #[test]
//     #[cfg(target_arch = "aarch64")]
//     fn test_barrett_reduce_neon() {
//         // 只在支持NEON的ARM平台上运行此测试
//         if !is_aarch64_feature_detected!("neon") {
//             return;
//         }
        
//         let input = vec![
//             0, MLKEM_Q, MLKEM_Q + 1, 5000, -1, -MLKEM_Q, 10000, 2000
//         ];
//         let expected = vec![
//             0, 0, 1, 5000 - MLKEM_Q, MLKEM_Q - 1, 0, 10000 - 3*MLKEM_Q, 2000
//         ];
        
//         unsafe {
//             let result = barrett_reduce_neon(&input);
//             assert_eq!(result, expected);
//         }
//     }
// }