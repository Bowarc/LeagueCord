/// Shamelessly stolen from <https://github.com/bowarc/crates> (time)
pub fn format(duration: web_time::Duration, mut prec: i8) -> String {
    const NANOS_IN_MICROSECOND: f64 = 1_000.0;
    const NANOS_IN_MILLISECOND: f64 = 1_000_000.0;
    const NANOS_IN_SECOND: f64 = 1_000_000_000.0;
    const NANOS_IN_MINUTE: f64 = NANOS_IN_SECOND * 60.0;
    const NANOS_IN_HOUR: f64 = NANOS_IN_MINUTE * 60.0;
    const NANOS_IN_DAY: f64 = NANOS_IN_HOUR * 24.0;
    const NANOS_IN_WEEK: f64 = NANOS_IN_DAY * 7.0;
    const NANOS_IN_YEAR: f64 = NANOS_IN_DAY * 365.0;

    let total_nanos = duration.as_nanos() as f64;

    if total_nanos < 1.0 {
        return format!("{:.0}ns", total_nanos.floor());
    }

    let mut remaining_nanos = total_nanos;
    let mut formatted_duration = String::new();

    if remaining_nanos >= NANOS_IN_YEAR && prec != 0 {
        prec -= 1;
        let years = remaining_nanos / NANOS_IN_YEAR;
        formatted_duration.push_str(&format!("{:.0}y ", years.floor()));
        remaining_nanos %= NANOS_IN_YEAR;
    }

    if remaining_nanos >= NANOS_IN_WEEK && prec != 0 {
        prec -= 1;
        let weeks = remaining_nanos / NANOS_IN_WEEK;
        formatted_duration.push_str(&format!("{:.0}w ", weeks.floor()));
        remaining_nanos %= NANOS_IN_WEEK;
    }

    if remaining_nanos >= NANOS_IN_DAY && prec != 0 {
        prec -= 1;
        let days = remaining_nanos / NANOS_IN_DAY;
        formatted_duration.push_str(&format!("{:.0}d ", days.floor()));
        remaining_nanos %= NANOS_IN_DAY;
    }

    if remaining_nanos >= NANOS_IN_HOUR && prec != 0 {
        prec -= 1;
        let hours = remaining_nanos / NANOS_IN_HOUR;
        formatted_duration.push_str(&format!("{:.0}h ", hours.floor()));
        remaining_nanos %= NANOS_IN_HOUR;
    }

    if remaining_nanos >= NANOS_IN_MINUTE && prec != 0 {
        prec -= 1;
        let minutes = remaining_nanos / NANOS_IN_MINUTE;
        formatted_duration.push_str(&format!("{:.0}m ", minutes.floor()));
        remaining_nanos %= NANOS_IN_MINUTE;
    }

    if remaining_nanos >= NANOS_IN_SECOND && prec != 0 {
        prec -= 1;
        let seconds = remaining_nanos / NANOS_IN_SECOND;
        formatted_duration.push_str(&format!("{:.0}s ", seconds.floor()));
        remaining_nanos %= NANOS_IN_SECOND;
    }

    if remaining_nanos >= NANOS_IN_MILLISECOND && prec != 0 {
        prec -= 1;
        let milis = remaining_nanos / NANOS_IN_MILLISECOND;
        formatted_duration.push_str(&format!("{:.0}ms ", milis.floor()));
        remaining_nanos %= NANOS_IN_MILLISECOND;
    }

    if remaining_nanos >= NANOS_IN_MICROSECOND && prec != 0 {
        prec -= 1;
        let micro = remaining_nanos / NANOS_IN_MICROSECOND;
        formatted_duration.push_str(&format!("{:.0}µs ", micro.floor()));
        remaining_nanos %= NANOS_IN_MICROSECOND;
    }

    if remaining_nanos > 0.0 && prec != 0 {
        formatted_duration.push_str(&format!("{:.0}ns", remaining_nanos.floor()));
    }

    formatted_duration.trim().to_string()
}
