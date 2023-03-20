void main() {
  int x = 10;
  int y = 20;
  print("Hello world!");
  print(addThenPrint(x, y));
  print("${divideNullOnZero(x, y)}");
}

String addThenPrint(int x, int y) {
  // dolla syntax
  return "${x + y}";
}

double? divideNullOnZero(int numerator, int denominator) {
  switch(denominator) {
    case 0: return null;  // nullable (cringe)
    default: return numerator / denominator;
  } 
}