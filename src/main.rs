use std::io;
use std::error::Error;
use csv;
struct MortgageParam {
    house_val: f64,
    down_payment: f64,
    interest_rate: f64,
    loan_years: f64,
}

struct Bills {
    utilities: f64,
    car_insur: f64,
    internet: f64,
    misc_bill:f64,
    state: char,
}

fn main()
{
    let mortgage = get_mortgage_details();
    let bills  = get_bills();
    println!(" ");
    println!("House value {}", mortgage.house_val);
    println!("Down Payment {}", mortgage.down_payment);
    println!("Interest Rate {}", mortgage.interest_rate);
    println!("Loan Years {}", mortgage.loan_years);
    println!("Utilities {}", bills.utilities);
    println!("Car Insurance {}", bills.car_insur);
    println!("Internet {}", bills.internet);
    println!("Misc Bill {}", bills.misc_bill);    
    println!("State {}", bills.state);
    println!("Monthly Pay {:.1}", calc_monthly_payment(&mortgage));

    let monthly_interest= mortgage.interest_rate / 12f64;
    let loan_amount = mortgage.house_val - mortgage.down_payment;
    let totalInterest = 0f64;
    let totalPrincipal = 0f64;
    let mut initalPMI = false;

    if(loan_amount > (mortgage.house_val* 0.8f64))
    {
        initalPMI = true;
    }
    if let Err(e) = write_to_file("./output.csv", &mortgage, &bills) 
    {
        eprint!("{}",e)
    }
    
 }

fn write_to_file(path: &str, mortgage: &MortgageParam, bills: &Bills) -> Result<(), Box<dyn Error>> 
{
    let mut writer = csv::Writer::from_path(path)?;
    writer.write_record(&[mortgage.house_val.to_string()])?;

    writer.flush()?;
    Ok(())
}

 fn get_mortgage_details() -> MortgageParam
 {
    MortgageParam {
        house_val: get_house_value(),
        down_payment: get_down_payment(),
        interest_rate: get_interest_rate(),
        loan_years: get_loan_years(),
    }
}

fn get_house_value() -> f64
{
    println!("Enter value of the house");
    let mut house_value = String::new();
    io::stdin()
        .read_line(&mut house_value)
        .expect("failed to read line");
    let val: f64 =  house_value
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;
}



fn get_down_payment() -> f64
{
    println!("Enter Down payment amount");
    let mut down_payment = String::new();
    io::stdin()
        .read_line(&mut down_payment)
        .expect("failed to read line");
    let val: f64 =  down_payment
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;
}


fn get_interest_rate() -> f64
{
    println!("Enter interest rate amount");
    let mut interest_rate = String::new();
    io::stdin()
        .read_line(&mut interest_rate)
        .expect("failed to read line");
    let val:f64 =  interest_rate
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val/100f64;
}


fn get_loan_years() -> f64
{
    println!("Enter years of loan");
    let mut interest_rate = String::new();
    io::stdin()
        .read_line(&mut interest_rate)
        .expect("failed to read line");
    let val:f64 =  interest_rate
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;
}

fn calc_monthly_payment(mortgage: &MortgageParam) -> f64 
{
    let monthly_interest = mortgage.interest_rate / 12f64;
    
    let discount_factor = (( (1f64 + monthly_interest).powf(360f64))-1f64)/(monthly_interest*((1f64 + monthly_interest).powf(360f64)));
    let loan_amount = mortgage.house_val - mortgage.down_payment;
    let monthly_pay =  (loan_amount/discount_factor).ceil();
    return monthly_pay;

}


fn get_bills() -> Bills
{
   Bills {
       utilities: get_utilities(),
       car_insur: get_car_insur(),
       internet: get_internet(),
       misc_bill: get_misc_bill(),
       state: get_state(),
   }
}

fn get_utilities() -> f64
{
    println!("Enter utilities cost");
    let mut utilities = String::new();
    io::stdin()
        .read_line(&mut utilities)
        .expect("failed to read line");
    let val:f64 =  utilities
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;    
}

fn get_car_insur() -> f64
{
    println!("Enter car insurance cost");
    let mut car_insur = String::new();
    io::stdin()
        .read_line(&mut car_insur)
        .expect("failed to read line");
    let val:f64 =  car_insur
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;     
}


fn get_internet() -> f64
{
    println!("Enter internet cost");
    let mut internet = String::new();
    io::stdin()
        .read_line(&mut internet)
        .expect("failed to read line");
    let val:f64 =  internet
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;     
}

fn get_misc_bill() -> f64
{
    println!("Enter misc bills cost");
    let mut misc = String::new();
    io::stdin()
        .read_line(&mut misc)
        .expect("failed to read line");
    let val:f64 =  misc
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val;     
}

fn get_state() -> char
{
    println!("Enter state N/M");
    let mut state = String::new();
    io::stdin()
        .read_line(&mut state)
        .expect("failed to read line");
    let val:char =  state
            .trim()
            .parse()
            .expect("Thats not a valid value");
    return val; 
}

fn get_house_insur(state: char) ->f64
{
    if(state == 'N' || state == 'n')
    {
        return 791f64 / 12f64;
    } else {
        return 1307f64 / 12f64;
    }
}

fn get_avg_property(state: char, mortgage: MortgageParam) -> f64
{
    if(state == 'N' || state == 'n')
    {
        return (0.0205f64 * (mortgage.house_val)) / 12f64;
    } else {
        return (0.0117f64 * (mortgage.house_val)) / 12f64;
    }
}