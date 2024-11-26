# Week 48

You're organizing a family meal. Given a list of dishes and their respective preparation times, return the minimum number of hours required to prepare all dishes if you can cook up to two dishes simultaneously. If a dish takes longer than the remaining time of the current hour, it will be moved to the next hour.

Examples:

```
// Using number of minutes for prep time

mealPrep([120])
2 // one single long dish

mealPrep([30, 30, 30, 20])
2 // multiple shorter dishes

mealPrep([30, 25, 45, 30, 60, 15])
3 // many dishes with varying times
```

# Notes

Redefined the problem to not include the remaining time of the current hour
logic, as I not really got that part lol. I just solved it in the shortest time
possible. This is also assuming that the dishes need to be prepped in order.
