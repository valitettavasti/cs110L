use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
    /*
    let mut list_string:LinkedList<String> = LinkedList::new();
    assert!(list_string.is_empty());
    assert_eq!(list_string.get_size(),0);
    list_string.push_front("aaa".to_string());
    list_string.push_front("bbb".to_string());
    println!("{}", list_string);
    println!("list size: {}", list_string.get_size());
    println!("top element: {}", list_string.pop_front().unwrap());
    println!("{}", list_string);
    println!("size: {}", list_string.get_size());
    */

}

#[cfg(test)]
    #[test]
    fn test_clone(){
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string());
    let list_clone = list.clone();
    println!("size: {}", list.get_size());
    println!("size: {}", list_clone.get_size());
    println!("{}", list.to_string());
    }

    #[test]
    fn test_partial_eq(){
        let mut list: LinkedList<u32> = LinkedList::new();
        for i in 1..12 {
            list.push_front(i);
        }
        let mut list_clone = list.clone();
        println!("{}",list.eq(&list_clone));
        list_clone.push_front(15);
        println!("{}",list.eq(&list_clone));
    }
