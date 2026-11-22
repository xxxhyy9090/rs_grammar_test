use crate::sort::merge_sort::merge_sort;
use crate::sort::quick_sort::quick_sort;

pub fn bubble_sort(num:&mut [i32]){
        let n=num.len();
        for i in 0..n{
            for j in 0..n-1-i{
                if num[j]>num[j+1] {
                    num.swap(j,j+1);
                }
            }
        }
    }
pub fn selection_sort(num:&mut [i32]){
    let n=num.len();
    for i in 0..n{
        let mut min = i;
        for j in i+1..n{
            if num[j] < num[min] {
                min = j;
            }
        }
        if min !=i{
            num.swap(i, min);
        }
    }
}
pub fn insertion_sort(num:&mut [i32]){
    let n=num.len();
    for i in 1..n{
        let mut j=i;
        while j>0 && num[j-1]>num[j] {
            num.swap(j-1,j);
            j=j-1;
        }
    }
}
pub mod merge_sort{
    pub fn merge_sort(num:&mut [i32]){
        let len=num.len();
        if len<=1{
            return;
        }
        let mid_num=len/2;
        let mut right=num[mid_num..].to_vec();
        let mut left=num[..mid_num].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);

        merge(num,&left,&right);
    }
    fn merge(num:&mut [i32],left:&[i32], right: &[i32]){
        let mut left_prt=0;
        let mut right_prt=0;
        let mut num_prt=0;

        while left_prt < left.len() && right_prt < right.len(){
            if left[left_prt]<=right[right_prt]{
                num[num_prt]=left[left_prt];
                left_prt+=1;
            }
            else{
                num[num_prt]=right[right_prt];
                right_prt+=1;
            }
            num_prt+=1;
        }
        while left_prt<left.len(){
            num[num_prt]=left[left_prt];
            left_prt+=1;
            num_prt+=1;
        }
        while right_prt<right.len(){
            num[num_prt]=right[right_prt];
            right_prt+=1;
            num_prt+=1;
        }
    }
}
pub mod quick_sort{
    pub fn partition(num:&mut [i32])-> usize{

        let pivot=num[num.len()-1];
        let mut i=0;
        for j in 0..num.len()-1{
            if num[j]<pivot{
                num.swap(i,j);
                i+=1;
            }
        }
        num.swap(i,num.len()-1);
        i
    }
    pub fn quick_sort(num:&mut [i32]){
        if num.len()<=1{
            return;
        }
        let pivot_num=partition(num);
        let(left,right)=num.split_at_mut(pivot_num);
        quick_sort(left);
        quick_sort(&mut right[1..]);
    }
}

    pub enum SortMode{
        BubbleSort,
        SelectionSort,
        InsertionSort,
        MergeSort,
        QuickSort,
    }
    pub type SortFn=fn(&mut [i32]);
    pub fn get_sort_fn(sort_mod: SortMode) -> SortFn {
        match sort_mod{
            SortMode::BubbleSort => bubble_sort,
            SortMode::SelectionSort => selection_sort,
            SortMode::InsertionSort => insertion_sort,
            SortMode::MergeSort => merge_sort,
            SortMode::QuickSort => quick_sort,
        }
    }
    pub fn parse_sort_mod(sort_mod:&str)->Option<SortMode>{
        match sort_mod{
            "bubble"=>Some(SortMode::BubbleSort),
            "selection"=>Some(SortMode::SelectionSort),
            "insertion"=>Some(SortMode::InsertionSort),
            "merge"=>Some(SortMode::MergeSort),
            "quick"=>Some(SortMode::QuickSort),
            _=>None,
        }
    }
    pub fn print_menu(){
            println!("Please enter your Vec");
            println!("For example:");
            println!("  bubble    -");
            println!("  selection -");
            println!("  insertion -");
            println!("  merge     -");
            println!("  quick     -");
            println!("  q         -");
            println!("Choice the sort mod::");
        }


