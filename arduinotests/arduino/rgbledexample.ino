/*
I am not sure how this program works it seems to be using ports 9-11
for RGB output - these seem to be digital out pins but they are not 
connected to anything.

I think this code (I got it from a code grepper result) is designed 
to be wired with a 4 pin RGB LED and a breadboard for example.

See https://arduinogetstarted.com/tutorials/arduino-rgb-led
*/

// Define constants for the output pins
const int redPin = 11;
const int greenPin = 10;
const int bluePin = 9;

void setup() {
  // Sets the RGB pins to off
  setColourRgb(0, 0, 0);
}

void loop() {
  unsigned int rgbColour[3];
  // Start off with red.
  rgbColour[0] = 255;
  rgbColour[1] = 0;
  rgbColour[2] = 0;  
  // Choose the colours to increment and decrement.
  for (int decColour = 0; decColour < 3; decColour += 1) {
    unsigned int incColour;

    // Sets an RGB value to increase based on the one to decrease
    if (decColour == 2) { incColour = 0; } else { incColour += 1; }
    // ^ Replace the ternary expr below
    //int incColour = decColour == 2 ? 0 : decColour + 1;

    // cross-fade the two colours.
    for (int i = 0; i < 255; i += 1) {
      // For range 0-255, increase or decrease two of the three colour values
      rgbColour[decColour] -= 1;
      rgbColour[incColour] += 1;

      // Set the RGB to the pins using setColourRgb
      setColourRgb(rgbColour[0], rgbColour[1], rgbColour[2]);
      
      // Delay of 5ms
      delay(5);
    }
  }
}

void setColourRgb(unsigned int red, unsigned int green, unsigned int blue) {
  /*
  This is a function that takes in 3 unsigned ints and writes those values 
  to the correct pins. This could be improved though because the values of
  the ints are not checked so something larger than 255 could be unsafe.
  */
  analogWrite(redPin, red);
  analogWrite(greenPin, green);
  analogWrite(bluePin, blue);
}