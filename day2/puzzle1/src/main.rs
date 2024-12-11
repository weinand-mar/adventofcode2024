fn main() {
    let list = utils::read_file_into_list_of_list("day2/puzzle1/list");

    let result = list
        .iter()
        .map(|x| is_report_safe(x, x.len()))
        .filter(|&x| x == 1)
        .count();

    print!("{}", result);
}

fn is_report_safe(report: &Vec<i32>, initLen: usize) -> i32 {
    let mut increasing = true;
    let mut decreasing = true;
    let mut differ = true;

    for i in 1..report.len() {
        if report[i - 1] > report[i] {
            increasing = false;
        } else if report[i - 1] < report[i] {
            decreasing = false;
        }
        if (report[i - 1] - report[i]).abs() == 0 || (report[i - 1] - report[i]).abs() > 3 {
            differ = false;
        }
    }

    if differ && (increasing || decreasing) {
        println!("{:?}", report);
        return 1;
    }

    if report.len() == initLen {
        return check_sub_reports_safe(report, initLen);
    }

    return 0;
}

fn check_sub_reports_safe(report: &Vec<i32>, initLen: usize) -> i32 {
    for x in 0..report.len() {
        let sub_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != x)
            .map(|(_, e)| *e)
            .collect();

        if is_report_safe(&sub_report, initLen) == 1 {
            return 1;
        }
    }
    return 0;
}
