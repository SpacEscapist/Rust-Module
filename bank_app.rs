pub fn bank_app() {
    // Use the standard IO module to capture User Input
    use std::io;

    // Initialize variables
    let mut checking_total = 0;
    let mut savings_total = 0;

    // Declare user input variable
    let mut user_input = String::new();

    // Present User with Welcome message
    println!("\nWelcome to the Banking App!");

    // While loop to continue asking user for input
    while user_input != "5" {
        // Present user with Main Menu options
        println!("\n----- Main Menu -----\n1. Deposit\n2. Withdraw\n3. Transfer Money\n4. View Account Balances\n5. Quit program\n---------------------\n");
        // Get user input
        println!("Please enter a number:");
        io::stdin().read_line(&mut user_input).unwrap();

        // Deposit ===================================================================================
        if user_input.trim() == "1" {
            // Clear user input
            user_input.clear();
            // Get user input
            println!("\nWhich account would you like to DEPOSIT into?\n1. Checking\n2. Savings");
            io::stdin().read_line(&mut user_input).unwrap();
            // Deposit to Checking
            if user_input.trim() == "1" {
                // Ask user how much to deposit
                println!("\nHow much would you like to DEPOSIT to Checking?");
                // Get response
                fn get_input() -> String {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Failed");
                    buffer
                }
                // Set user response to a variable and convert input to an Integer
                let c = get_input().trim().parse::<i64>().unwrap();
                // Add deposit amount to running total
                checking_total = checking_total + c;
                // Display total
                println!("\n==================================");
                println!("You DEPOSITED --- ${}", c);
                println!("\nYour Checking Balance is:  ${}", checking_total);
                println!("==================================");
                // Clear user input
                user_input.clear();
            }
            // Deposit to Savings
            if user_input.trim() == "2" {
                // Ask user how much to deposit
                println!("\nHow much would you like to DEPOSIT to Savings?");
                // Get response
                fn get_input() -> String {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Failed");
                    buffer
                }
                // Set user response to a variable and convert input to an Integer
                let s = get_input().trim().parse::<i64>().unwrap();
                // Add deposit amount to running total
                savings_total = savings_total + s;
                // Display total
                println!("\n==================================");
                println!("You DEPOSITED --- ${}", s);
                println!("\nYour Savings Balance is:  ${}", savings_total);
                println!("==================================");
                // Clear user input
                user_input.clear();
            }
        }
        // Withdraw ===================================================================================
        else if user_input.trim() == "2" {
            // Clear user input
            user_input.clear();
            // Get user input
            println!("\nWhich account would you like to WITHDRAW from?\n1. Checking\n2. Savings");
            io::stdin().read_line(&mut user_input).unwrap();
            // Withdraw from Checking
            if user_input.trim() == "1" {
                // Ask user how much to withdraw
                println!("\nHow much would you like to WITHDRAW from Checking?");
                // Get response
                fn get_input() -> String {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Failed");
                    buffer
                }
                // Set user response to a variable and convert input to an Integer
                let c = get_input().trim().parse::<i64>().unwrap();
                // Test if there's enough money in account.
                // If so, subtract withdraw amount from running total
                if checking_total - c >= 0 {
                    checking_total = checking_total - c;
                    // Display total
                    println!("\n==================================");
                    println!("You WITHDREW --- ${}", c);
                    println!("\nYour Checking Balance is:  ${}", checking_total);
                    println!("==================================");
                } else {
                    println!("\n================================================");
                    println!("Cannot complete transaction: INSUFFICIENT FUNDS");
                    println!("\nYour Checking Balance is:  ${}", checking_total);
                    println!("================================================");
                }
                // Clear user input
                user_input.clear();
            }
            // Withdraw from Savings
            else if user_input.trim() == "2" {
                // Ask user how much to withdraw
                println!("\nHow much would you like to WITHDRAW from Savings?");
                // Get response
                fn get_input() -> String {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Failed");
                    buffer
                }
                // Set user response to a variable and convert input to an Integer
                let s = get_input().trim().parse::<i64>().unwrap();
                // Test if there's enough money in account.
                // If so, subtract withdraw amount from running total
                if savings_total - s >= 0 {
                    savings_total = savings_total - s;
                    // Display total
                    println!("\n==================================");
                    println!("You WITHDREW --- ${}", s);
                    println!("\nYour Savings Balance is:  ${}", savings_total);
                    println!("==================================");
                } else {
                    println!("\n================================================");
                    println!("Cannot complete transaction: INSUFFICIENT FUNDS");
                    println!("\nYour Savings Balance is:  ${}", savings_total);
                    println!("================================================");
                }
                // Clear user input
                user_input.clear();
            }
        }
        // Transfer Money ===================================================================================
        else if user_input.trim() == "3" {
            // Clear user input
            user_input.clear();
            // Get user input
            println!("\nHow would you like to TRANSFER funds?\n1. Checking --> Savings\n2. Savings --> Checking");
            io::stdin().read_line(&mut user_input).unwrap();
            // Transfer from Checking
            if user_input.trim() == "1" {
                // Ask user how much to transfer
                println!("\nHow much would you like to TRANSFER from Checking to Savings?");
                // Get response
                fn get_input() -> String {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Failed");
                    buffer
                }
                // Set user response to a variable and convert input to an Integer
                let c = get_input().trim().parse::<i64>().unwrap();
                // Test if there's enough money in account.
                // If so, subtract transfer amount from running
                // total of Checking and add to Savings
                if checking_total - c >= 0 {
                    checking_total = checking_total - c;
                    savings_total = savings_total + c;
                    // Display total
                    println!("\n===================================================");
                    println!("You TRANSFERRED from Checking to Savings --- ${}", c);
                    println!("\nYour Checking Balance is:  ${}", checking_total);
                    println!("Your Savings Balance is:  ${}", savings_total);
                    println!("===================================================");
                } else {
                    println!("\n================================================");
                    println!("Cannot complete transaction: INSUFFICIENT FUNDS");
                    println!("\nYour Checking Balance is:  ${}", checking_total);
                    println!("================================================");
                }
                // Clear user input
                user_input.clear();
            }
            // Transfer from Savings
            else if user_input.trim() == "2" {
                // Ask user how much to transfer
                println!("\nHow much would you like to TRANSFER from Savings to Checking?");
                // Get response
                fn get_input() -> String {
                    let mut buffer = String::new();
                    std::io::stdin().read_line(&mut buffer).expect("Failed");
                    buffer
                }
                // Set user response to a variable and convert input to an Integer
                let s = get_input().trim().parse::<i64>().unwrap();
                // Test if there's enough money in account.
                // If so, subtract transfer amount from running
                // total of Savings and add to Checking
                if savings_total - s >= 0 {
                    savings_total = savings_total - s;
                    checking_total = checking_total + s;
                    // Display total
                    println!("\n===================================================");
                    println!("You TRANSFERRED from Savings to Checking --- ${}", s);
                    println!("\nYour Checking Balance is:  ${}", checking_total);
                    println!("Your Savings Balance is:  ${}", savings_total);
                    println!("===================================================");
                } else {
                    println!("\n================================================");
                    println!("Cannot complete transaction: INSUFFICIENT FUNDS");
                    println!("\nYour Savings Balance is:  ${}", savings_total);
                    println!("================================================");
                }
                // Clear user input
                user_input.clear();
            }
        }
        // View Accounts ===================================================================================
        else if user_input.trim() == "4" {
            println!("\n=============================");
            println!(
                "Your Account Balances:\n\nChecking: ${}\nSavings: ${}",
                checking_total, savings_total
            );
            println!("=============================");
            // Clear user input
            user_input.clear();
        }
        // Exit Program ===================================================================================
        else if user_input.trim() == "5" {
            println!("\nQuitting program...\n");
            break;
        }
        // Catch user input error
        else {
            println!("\n============================\nPlease enter a valid input\n============================");
            // Clear user input
            user_input.clear();
        }
    }
}
