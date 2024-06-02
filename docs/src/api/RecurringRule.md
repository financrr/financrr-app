# Recurring rule

A recurring rule is a rule that defines when and how often a recurring transaction should be executed.  
Structure of a recurring rule:

```json
{
    "cron_pattern": {
        "day_of_month": "*",
        "month": "*",
        "day_of_week": "*"
    }
}
```

This rule will execute the transaction every day.

- `day_of_month`: The day of the month when the transaction should be executed.
    - `*`: Every day of the month.
    - `1-31`: The day of the month.
- `month`: The month when the transaction should be executed.
    - `*`: Every month.
    - `1-12`: The month.
- `day_of_week`: The day of the week when the transaction should be executed.
    - `*`: Every day of the week.
    - `1-7`: The day of the week. (1 is Monday, ..., 6 is Saturday, 7 is Sunday)

We use [croner-rust](https://github.com/hexagon/croner-rust) to parse the recurring rule.  
Check out their [repository](https://github.com/hexagon/croner-rust) to see what is supported.

### Special rules

There are some special rules that can be used instead of the cron pattern.  
The special rules are:

- `@yearly`: Execute the transaction every year.
- `@annually`: Execute the transaction every year.
- `@monthly`: Execute the transaction every month.
- `@weekly`: Execute the transaction every week.
- `@daily`: Execute the transaction every day.

### How to use it easily

You should be able to use a cron builder to create the recurring rule.  
Simply build the cron and extract the `day_of_month`, `month`, and `day_of_week` from the cron string.

## Examples

### repeat every day

```json
{
    "cron_pattern": {
        "day_of_month": "*",
        "month": "*",
        "day_of_week": "*"
    }
}
```

### repeat every 4th day

```json
{
    "cron_pattern": {
        "day_of_month": "*",
        "month": "*",
        "day_of_week": "*/4"
    }
}
```

### repeat every 2nd week on monday and friday

```json
{
    "cron_pattern": {
        "day_of_month": "*",
        "month": "*",
        "day_of_week": "1,5"
    }
}
```

### repeat every month

```json
{
    "special": "@monthly"
}
```

### repeat every 3rd month on the 2nd monday

```json
{
    "cron_pattern": {
        "day_of_month": "*",
        "month": "*/3",
        "day_of_week": "1#2"
    }
}
```

### repeat every year on january and august

```json
{
    "cron_pattern": {
        "day_of_month": "*",
        "month": "1,8",
        "day_of_week": "*"
    }
}
```
