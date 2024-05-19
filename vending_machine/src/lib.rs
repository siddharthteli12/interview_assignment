


/*
A vending machine has the following denominations: 1c, 5c, 10c, 25c, 50c, and $1. Your task is to write a program that will be used in a vending machine to return change. Assume that the vending machine will always want to return the least number of coins or notes. Devise a function getChange(M, P) where M is how much money was inserted into the machine and P the price of the item selected, that returns an array of integers representing the number of each denomination to return.

Example:
getChange(5, 0.99) // should return [1,0,0,0,0,4]

getChange(3.14, 1.99) // should return [0,1,1,0,0,1]
getChange(3, 0.01) // should return [4,0,2,1,1,2]
getChange(4, 3.14) // should return [1,0,1,1,1,0]
getChange(0.45, 0.34) // should return [1,0,1,0,0,0]

*/

#[allow(dead_code)]
fn get_change(cash: f64, cost: f64) -> [u16; 6] {
    let cash = (cash * 100.0) as u16;
    let cost = (cost * 100.0) as u16;

    let mut change = cash - cost;
    let mut result = [0_u16; 6];
    let currencys = [100, 50, 25, 10, 5, 1];
    for (n, &currency) in currencys.iter().enumerate() {
        if change == 0 {
            break;
        }
        if change >= currency {
            let count = change / currency;
            change %= currency;
            result[n] = count;
        }
    }
    result.reverse();
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert!([1,0,0,0,0,4] == get_change(5.0, 0.99));
    }

    #[test]
    fn test_2() {
        assert!([0,1,1,0,0,1] == get_change(3.14, 1.99));
    }

    #[test]
    fn test_3() {
        assert!([4,0,2,1,1,2] == get_change(3.0, 0.01));
    }

    #[test]
    fn test_4() {
        assert!([1,0,1,1,1,0] == get_change(4.0, 3.14));
    }

    #[test]
    fn test_5() {
        assert!([1,0,1,0,0,0] == get_change(0.45, 0.34));
    }

}
