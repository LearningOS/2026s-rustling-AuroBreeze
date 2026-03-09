/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd + std::clone::Clone>(array: &mut [T]) {
    //TODO
    if array.len() <= 1 {
        return;
    }
    let mut smallest = array[0].clone();
    let mut addr = 0;
    let mut temp = 0;

    while true {
        for i in temp..array.len() {
            if array[i] <= smallest {
                smallest = array[i].clone();
                addr = i;
            }
        }
        array.swap(addr, temp);
        temp += 1;
        if temp == array.len() {
            break;
        }

        smallest = array[temp].clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
