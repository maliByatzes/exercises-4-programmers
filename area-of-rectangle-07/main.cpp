// Create a program that calculates the area of a room. Prompt
// the user for the length and width of the room in feet. Then
// display the area in both square feet and square meters.
//
// Example Output
// What is the length of the room in feet? 15
// What is the width of the room in feet? 20
// You entered dimensions of 15 feet by 20 feet.
// The area is
// 300 square feet
// 27.871 square meters
//
// The formula for this conversion is
// m^2 = f^2 × 0.09290304
//
// Constraints
// • Keep the calculations separate from the output.
// • Use a constant to hold the conversion factor.

#include <iostream>

int main() {
  const auto conversion_factor{0.09290304};

  int length{0};
  std::cout << "What is the length of the room in feet? ";
  std::cin >> length;

  int width{0};
  std::cout << "What is the width of the room in feet? ";
  std::cin >> width;

  std::cout << "You entered dimensions of " << length << " feet by " << width
            << " feet.\n";

  auto area = length * width;

  std::cout << "The area is " << area << " square feet\n";

  std::cout << area * conversion_factor << " square meters\n";

  return 0;
}
