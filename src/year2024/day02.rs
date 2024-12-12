fn parse(contents: &String) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in contents.lines() {
        let characters = line
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let mut report: Vec<i32> = Vec::new();

        for character in characters {
            report.push(character.parse::<i32>().unwrap());
        }

        reports.push(report);
    }

    return reports;
}

fn part1(reports: &Vec<Vec<i32>>) {
    let mut report_safeties: Vec<bool> = Vec::new();

    for report in reports {
        let is_asc = (report[1] - report[0]).is_positive();
        let mut is_safe = true;

        for n in 1..report.len() {
            let difference = report[n] - report[n - 1];
            let is_asc_new = difference.is_positive();
            let is_increase_safe = (1 <= difference.abs()) & (difference.abs() <= 3);

            if is_asc_new != is_asc || !is_increase_safe {
                is_safe = false;
                break;
            }
        }
        report_safeties.push(is_safe);
    }

    let amount_safe = report_safeties.iter().filter(|&x| *x).count();
    println!("Part 1: {amount_safe}");
}

fn is_report_safe(report: Vec<i32>, dampener: bool) -> bool {
    let is_asc = (report[1] - report[0]).is_positive();
    let mut is_safe = true;

    for n in 1..report.len() {
        let difference: i32;
        difference = report[n] - report[n - 1];
        let is_asc_new = difference.is_positive();
        let is_increase_safe = (1 <= difference.abs()) & (difference.abs() <= 3);

        if is_asc_new != is_asc || !is_increase_safe {
            if dampener {
                // Ignore left value
                let mut report_1 = report.clone();
                report_1.remove(n - 1);

                if is_report_safe(report_1, false) {
                    break;
                }

                // Ignore right value
                let mut report_2 = report.clone();
                report_2.remove(n);

                if is_report_safe(report_2, false) {
                    break;
                }

                // Ignore first value to check if asc/desc was the issue
                let mut report_3 = report.clone();
                report_3.remove(0);

                if is_report_safe(report_3, false) {
                    break;
                }
            }
            is_safe = false;
            break;
        }
    }
    return is_safe;
}

fn part2(reports: &Vec<Vec<i32>>) {
    let mut report_safeties: Vec<bool> = Vec::new();

    for report in reports {
        let is_safe = is_report_safe(report.clone(), true);
        report_safeties.push(is_safe);
    }

    let amount_safe = report_safeties.iter().filter(|&x| *x).count();
    println!("Part 2: {amount_safe}");
}

pub fn day2(contents: &String) {
    let reports = parse(contents);
    part1(&reports);
    part2(&reports);
}
