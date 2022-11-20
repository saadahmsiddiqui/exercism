function colorResistance(color: string): number {
  switch (color) {
    case "black":
      return 0;
    case "brown":
      return 1
    case "red":
      return 2
    case "orange": 
      return 3;
    case "yellow": 
      return 4;
    case "green": 
      return 5;
    case "blue": 
      return 6;
    case "violet": 
      return 7;
    case "grey": 
      return 8;
    case "white": 
      return 9;
    default:
      throw new Error("Invalid Color");
  }
}


export function decodedValue(colors: string[]): number {
  let resistance = "";
  for (const color of colors) {
    resistance += colorResistance(color);
  }

  return Number(resistance)
}
