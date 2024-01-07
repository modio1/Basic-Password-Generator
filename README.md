# Rust Password Generator

This is a basic password generator written in Rust. It prompts the user to specify the desired length of the password and then generates a random password containing alphanumeric characters and symbols.

## How to Use

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/modio1/Basic-Password-Generator.git
   ```

2. Navigate to the project directory:

   ```bash
   cd Basic-Password-Generator
   ```

3. Run the Rust program:

   ```bash
   cargo run
   ```

   The program will prompt you to enter the desired length of the password.

4. Enter the desired password length and press Enter.

5. The program will generate and display a random password.

## Example

```bash
How many characters would you like the password to be? 
12
Your password is aB7&gJ2#mFpW
```

Feel free to customize the character set used in the password generation by modifying the `char_set` array in the code.

## Dependencies

- `rand`: A crate for generating random numbers.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

