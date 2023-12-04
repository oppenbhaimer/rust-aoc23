export $(cat .env | xargs)

curl -v --cookie $AOC23_COOKIE https://adventofcode.com/2023/day/$1/input -o input/input_$1;
