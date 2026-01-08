// Definition for singly-linked list.
 #[derive(PartialEq, Eq, Clone, Debug)]
 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 
 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
     ListNode {
       next: None,
       val
     }
   }
 }

fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // parse input to numbers

    let mut n1: i32 = 0;
    let mut n2: i32 = 0;

    let mut sublist1 = l1;
    let mut sublist2 = l2;

    let mut place_index = 0;

    while let Some((a, b)) = sublist1.zip(sublist2) {

        let place_power = 10_i32.pow(place_index);
        place_index += 1;
        n1 += a.val * place_power;
        n2 += b.val * place_power;


        sublist1 = a.next;
        sublist2 = b.next;
    };

    let sum = n1 + n2;

    println!("Sum: {}", sum);


    // Reconstruct List

    let mut result_list: Option<Box<ListNode>> = None;

    for i in 0..place_index {
        println!("lol {}", i);
        
        result_list = Some(Box::new(ListNode { val: (sum / 10_i32.pow(place_index - i - 1)) % 10, next: result_list}));
    }
    

    result_list
    
}


//test cases in main function

// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.

fn main() {
    let l1 = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode { val: 3, next: None })) })) }));
    let l2 = Some(Box::new(ListNode { val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 4, next: None })) })) }));
    
    let result = add_two_numbers(l1, l2);
    
    // Print the result list
    let mut current = &result;
    while let Some(node) = current {
        print!("{} ", node.val);
        current = &node.next;
    }
    // Expected output: 7 0 8
}

