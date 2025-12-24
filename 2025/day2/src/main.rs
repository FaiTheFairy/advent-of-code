use day2::{sum_of_ids_repeated_at_least_twice, sum_of_ids_repeated_exactly_twice};

fn main() {
    let input = "1090286-1131879,3259566-3404881,138124-175118,266204727-266361099,16765-24272,7657360692-7657593676,88857504-88926597,6869078-6903096,48444999-48532270,61427792-61580535,71-103,8077-10421,1920-2560,2-17,951-1259,34-50,28994-36978,1309-1822,9393918461-9393960770,89479-120899,834641-988077,5389718924-5389797353,34010076-34214499,5063-7100,607034-753348,19098586-19261191,125085556-125188689,39839-51927,3246-5037,174-260,439715-473176,187287-262190,348-535,58956-78301,4388160-4505757,512092-584994,13388753-13534387".to_string();
    let sum_invalid_ids_twice: u64 = sum_of_ids_repeated_exactly_twice(input.clone());
    println!(
        "Part 1. Sum of IDs that include digits repeated exactly twice: {:?}",
        sum_invalid_ids_twice
    );
    let sum_invalid_ids_at_least_twice: u64 = sum_of_ids_repeated_at_least_twice(input);
    println!(
        "Part 2. Sum of IDs that include digits repeated at least twice: {:?}",
        sum_invalid_ids_at_least_twice
    );
}
