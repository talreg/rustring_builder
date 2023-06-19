## String Builder - reusable char buffer
* The StringBuilder object allows you to add any object (including custom types)
as long as they implement the ToString trait.
* StringBuilder doesn't consume the object appended
* StringBuilder is not consumed by calling the to_string() method.
* StringBuilder can be appended to another StringBuilder.
## Consuming methods:
* Operators, such as + or += will consume the respective value.
* Iterator will also consume the value
* Direct access (nth) will **not** consume the value.