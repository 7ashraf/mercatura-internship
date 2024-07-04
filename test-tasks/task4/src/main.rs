struct FilterCondition{
    val: i32,
}

impl FilterCondition{
    fn is_match(&self, obj: i32) -> bool{
        obj == self.val
    }

}

fn custom_filter (collection: &mut Vec<i32>, filter: FilterCondition) -> Vec<i32>{
    let mut new_collection: Vec<i32> = vec![];
    for item in collection.iter(){
        if filter.is_match(*item){
            new_collection.push(*item);
        }
    }

    new_collection



}

fn main() {
    //replace <val> argument with the filtered number you wish to get
    let mut filter = FilterCondition{val: 0};
    let mut arr = vec![0,1,0,0,1]; // out: [0,0,0]
    let filtered_vec = custom_filter(&mut arr, filter);

    

    println!("{:?}", filtered_vec);
}
