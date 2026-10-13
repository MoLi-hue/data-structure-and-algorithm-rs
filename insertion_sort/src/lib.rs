pub trait Sorting {
    fn insertion_sort(&mut self);
}

impl<T: Ord> Sorting for [T] {
    fn insertion_sort(&mut self) {
        let len = self.len();

        for i in 0..len {
            for j in (0..i).rev() {
                if self[j] > self[j + 1] {
                    self.swap(j, j + 1);
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    //! 本模块下所有单元测试代码由 AI 生成

    use super::*;

    /// 测试：无序数组应被排序为增序
    #[test]
    fn test_basic_unsorted() {
        let mut v = [5, 2, 9, 1, 5, 6];
        v.insertion_sort();
        assert_eq!(v, [1, 2, 5, 5, 6, 9]);
    }

    /// 测试：已经有序的数组保持不变
    #[test]
    fn test_already_sorted() {
        let mut v = [1, 2, 3, 4, 5];
        v.insertion_sort();
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    /// 测试：逆序数组应被反转为增序
    #[test]
    fn test_reverse_sorted() {
        let mut v = [5, 4, 3, 2, 1];
        v.insertion_sort();
        assert_eq!(v, [1, 2, 3, 4, 5]);
    }

    /// 测试：包含重复元素的数组
    #[test]
    fn test_duplicates() {
        let mut v = [3, 1, 3, 2, 1, 2];
        v.insertion_sort();
        assert_eq!(v, [1, 1, 2, 2, 3, 3]);
    }

    /// 测试：空数组
    #[test]
    fn test_empty() {
        let mut v: [i32; 0] = [];
        v.insertion_sort();
        assert_eq!(v, []);
    }

    /// 测试：单元素数组
    #[test]
    fn test_single_element() {
        let mut v = [42];
        v.insertion_sort();
        assert_eq!(v, [42]);
    }

    /// 测试：两个元素（已排序）
    #[test]
    fn test_two_elements_sorted() {
        let mut v = [1, 2];
        v.insertion_sort();
        assert_eq!(v, [1, 2]);
    }

    /// 测试：两个元素（逆序）
    #[test]
    fn test_two_elements_reversed() {
        let mut v = [2, 1];
        v.insertion_sort();
        assert_eq!(v, [1, 2]);
    }

    /// 测试：所有元素相同
    #[test]
    fn test_all_equal() {
        let mut v = [7, 7, 7, 7, 7];
        v.insertion_sort();
        assert_eq!(v, [7, 7, 7, 7, 7]);
    }

    /// 测试：包含负数
    #[test]
    fn test_negative_numbers() {
        let mut v = [-3, 5, -1, 0, -10, 8];
        v.insertion_sort();
        assert_eq!(v, [-10, -3, -1, 0, 5, 8]);
    }

    /// 测试：极值（i32 边界）
    #[test]
    fn test_i32_extremes() {
        let mut v = [i32::MAX, i32::MIN, 0, -1, 1];
        v.insertion_sort();
        assert_eq!(v, [i32::MIN, -1, 0, 1, i32::MAX]);
    }

    /// 测试：字符串切片（验证泛型约束 T: Ord）
    #[test]
    fn test_strings() {
        let mut v = ["banana", "apple", "cherry", "date"];
        v.insertion_sort();
        assert_eq!(v, ["apple", "banana", "cherry", "date"]);
    }

    /// 测试：通过 Vec 调用（Vec 可 Deref 为 [T]）
    #[test]
    fn test_with_vec() {
        let mut v = vec![9, 4, 7, 1, 3];
        v.insertion_sort();
        assert_eq!(v, vec![1, 3, 4, 7, 9]);
    }

    /// 测试：排序后数组确实为增序（属性测试思想）
    #[test]
    fn test_result_is_sorted() {
        let mut v = [10, 3, 15, 7, 8, 1, 20, 5];
        v.insertion_sort();
        for i in 0..v.len().saturating_sub(1) {
            assert!(v[i] <= v[i + 1], "数组未按增序排列: {:?}", v);
        }
    }

    /// 测试：排序不改变数组长度
    #[test]
    fn test_length_preserved() {
        let mut v = [4, 2, 8, 1, 9, 3];
        let original_len = v.len();
        v.insertion_sort();
        assert_eq!(v.len(), original_len);
    }

    /// 测试：排序后元素总和不变（元素未丢失或重复）
    #[test]
    fn test_sum_preserved() {
        let mut v = [4, 2, 8, 1, 9, 3];
        let original_sum: i32 = v.iter().sum();
        v.insertion_sort();
        let sorted_sum: i32 = v.iter().sum();
        assert_eq!(original_sum, sorted_sum);
    }

    /// 测试：大数组（性能与正确性粗检）
    #[test]
    fn test_large_array() {
        let mut v: Vec<i32> = (0..100).rev().collect();
        v.insertion_sort();
        let expected: Vec<i32> = (0..100).collect();
        assert_eq!(v, expected);
    }
}
