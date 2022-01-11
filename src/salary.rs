#![allow(unused)]

use crate::input;

pub fn driver()  -> Result<bool, input::InputError> {
    let n = match input::get_int() {
        Ok(n) => n,
        Err(e) => {return Err(e)},
    };
    let mut numbers = match input::get_int_vec(n as usize) {
        Ok(numbers) => numbers,
        Err(e) => return Err(e),
    };

    let result : String = String::new();
    println!("{}", result);
    Ok(true)
}

// bool greaterOrEqual(const string &digit, const string &max) {
// if (digit + max >= max + digit) {
// return true;
// }
// return false;
// }
//
// string largestNumber(vector<string> numbers) {
// string result;
// std::stringstream str;
//
// while (!numbers.empty()) {
// string max("0");
// INT pos = 0;
// for (INT i = 0; i < numbers.size(); i++) {
// if (greaterOrEqual(numbers[i], max)) {
// max = numbers[i];
// pos = i;
// }
// }
// str << max;
// numbers.erase(numbers.begin() + pos);
// }
// str >> result;
// return result;
// }
//
// int main() {
// INT n;
// cin >> n;
// vector<string> numbers(n);
// for (auto &number: numbers) {
// cin >> number;
// }
// cout << largestNumber(numbers);
// return 0;
// }