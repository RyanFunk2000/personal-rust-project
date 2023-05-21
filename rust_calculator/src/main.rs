/*
 ============================================================================
 Name        : Rust Personal Project
 Author      : Ryan Funk
 Version     : 1.0
 Copyright   :
 Description : Video Game Time Bonus Calculator
 ============================================================================
 */

use std::io;

fn main() {
    println!("This program is designed to be used for the video game 'Sonic Unleashed' to calculate how the time bonus behaves for a given level.");
    println!("It is required that you enter results score data from two different play sessions of a single level.");
    println!();

    loop {
        println!();
        // Call and assign convertFinalTime and calculateScoreRate functions for first sample.
		println!("Please enter the results score data from the first play session.");
        let sample1_time: f64 = convert_final_time();
        let sample1_time_bonus: i32 = find_time_bonus();
        println!();

        // Call and assign convertFinalTime and calculateScoreRate functions again for second sample.
		println!("Please enter the results score data from the second play session.");
        let sample2_time: f64 = convert_final_time();
        let sample2_time_bonus: i32 = find_time_bonus();
        println!();

        // Call and assign calculateScoreRate function with the first and second sample's time and time bonus as parameters.
        let score_rate: i32 = calculate_score_rate((sample1_time,sample1_time_bonus,sample2_time,sample2_time_bonus));

        // Call and assign calculateStartPoints function with the first sample's time and time bonus and score rate assigned from calculateScoreRate as parameters.
        let max_time_bonus: i32 = calculate_start_points((sample1_time,sample1_time_bonus,score_rate));
    
        // Calculate when all time bonus points would be subtracted and be zero with returned values from calculateScoreRate and calculateStartPoints.
        let time_limit_seconds: f64 = f64::from(max_time_bonus / score_rate);

        // Call and assign calculateTimeLimit function with score rate assigned from calculateScoreRate and max_time_bonus assigned from calculateStartPoints as parameters.
        let time_limit_standard: String = format_time_limit(time_limit_seconds);

        // Display max_time_bonus from calculateScoreRate.
		println!("At 00:00:00, time bonus starts at {} points.", max_time_bonus);
		// Display score_rate from calculateScoreRate.
		println!("The rate at which time bonus subtracts is {} points per second.", score_rate);
		// Display both time_limit_seconds from calculation in main and time_limit_standard from formatTimeLimit.
		println!("The player has this amount of time to beat the stage before there is no time bonus remaining: {} or {} seconds.", time_limit_standard, time_limit_seconds);

        println!();
        // Prompt user to loop or break program.
        println!("Restart and make another calculation? y/n: ");
        let mut choice: String = String::new();
		io::stdin().read_line(&mut choice).expect("failed to readline");
        if choice != String::from("y") {
            break;
        }
    }
}

fn convert_final_time() -> f64 {
    /* Receive from the user the final time from one play
	session and convert the formatted time to seconds. */
    let mut time_str: String = String::new();

    println!("Enter 'TIME' in the game's format (##:##:##): ");
    io::stdin().read_line(&mut time_str).expect("failed to readline");

    // Split minutes, seconds, and milliseconds from input string into seperate variables.
    let time_split = time_str.split(":");
    let mut time_array: [i32; 3] = [0, 0, 0];

    let mut i: usize = 0;
    for number in time_split
    {
        // Convert current number from string to integer.
        let number: i32 = number.trim().parse::<i32>().unwrap();

        // Pass number into time array and iterate loop.
        time_array[i] = number;
        i += 1;
    }
    // Convert and return final time as seconds.
    return (f64::from(time_array[0])*60.0) + f64::from(time_array[1]) + (f64::from(time_array[2])*0.01);
}

fn find_time_bonus() -> i32 {
    /* Receive from the user the score from one play session and use that
	to find and return the remaining score received from time bonus. */

    // Receive each category of points on game's results screen.
    let rings: i32 = input_score("RINGS");
    let speed: i32 = input_score("SPEED");
    let enemy: i32 = input_score("ENEMY");
    let tricks: i32 = input_score("TRICKS");
    let total: i32 = input_score("TOTAL");

    return total - (rings+speed+enemy+tricks);
}

fn input_score(category : &str) -> i32 {
    /* Check user input for findTimeBonus to ensure
	it's a valid score before proceeding program. */
    let mut score: String = String::new();

    // Prompt the user to input the defined category of points displayed on the results screen from one play session.
    println!("Enter {}: ", category);
    io::stdin().read_line(&mut score).expect("failed to readline");

    // Return user input as an integer.
    return score.trim().parse().unwrap();
}

fn calculate_score_rate((sample1_time, sample1_time_bonus, sample2_time, sample2_time_bonus): (f64, i32, f64, i32)) -> i32 {
    /* Take final time samples and time bonus samples to calculate
	and return the rate at which points are subtracted per second. */
    
    // Find difference for time and time bonus score and guarantee they remain positive.
    let time_diff: f64 = (sample1_time - sample2_time).abs();
    let score_diff: i32 = (sample1_time_bonus - sample2_time_bonus).abs();

    // Return time difference divided by score difference for score rate.
    return (f64::from(score_diff) / time_diff).round() as i32;
}

fn calculate_start_points((time,time_bonus,score_rate): (f64, i32, i32)) -> i32 {
    /* Take time bonus and final time from one sample and score rate from calculateScoreRate to calculate
	and return the amount of points from time bonus that the player has at the beginning of the level. */
    
	// Find the amount of points lost due to time.
	let points_lost: f64 = time * f64::from(score_rate);
	// Return sum of time bonus that was lost and remaining time bonus received, rounded by multiple of 10.
    let start_points: f64 = ((points_lost + f64::from(time_bonus))/10.0).round()*10.0;
	return start_points as i32;
}

fn format_time_limit(time_limit_seconds: f64) -> String {
    /* Convert time limit from seconds into the format 'MINUTES:SECONDS:MILLISECONDS'. */

    // Split time_limit from seconds to seperate variables for milliseconds, seconds, and minutes.
    let m: i32 = (time_limit_seconds.floor() / 60.0) as i32;
    let s: i32 = (time_limit_seconds.floor() % 60.0) as i32;
    let ms: i32 = ((time_limit_seconds - time_limit_seconds.floor()) * 100.0) as i32;

    // Add split values into new array.
    let time_limit_format: Vec<String> = vec![m.to_string(), s.to_string(), ms.to_string()];

    return time_limit_format.join(":");
}