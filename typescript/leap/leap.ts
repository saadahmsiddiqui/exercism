export function isLeap(year: number) {
  return year % 4 === 0 ? (
    year % 100 === 0 ? (
      year % 400 === 0 ? true : false
    ) : true
  ) : false
}
