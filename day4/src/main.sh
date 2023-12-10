#!/bin/bash

parse_input() {
    local input_str="$1"
    local -n cards_ref=$2

    while IFS= read -r line; do
        local card_number=$(echo "$line" | awk '{print $2}')
        local numbers=$(echo "$line" | cut -d ':' -f 2)
        local winning_numbers=$(echo "$numbers" | cut -d '|' -f 1)
        local own_numbers=$(echo "$numbers" | cut -d '|' -f 2)

        cards_ref+=("$card_number:$winning_numbers:$own_numbers")
    done <<< "$input_str"
}

get_card_score() {
    local card=$1
    local own_numbers=$(echo "$card" | cut -d ':' -f 3)
    local winning_numbers=$(echo "$card" | cut -d ':' -f 2)

    local score=0
    for own_number in $own_numbers; do
        for winning_number in $winning_numbers; do
            if [ "$own_number" -eq "$winning_number" ]; then
                ((score++))
            fi
        done
    done

    echo $score
}

solve() {
    local input_str="$1"
    local -a cards
    parse_input "$input_str" cards

    local -a counts
    counts=($(yes 1 | head -n ${#cards[@]}))

    for i in "${!cards[@]}"; do
        local score=$(get_card_score "${cards[$i]}")
        if [ "$score" -gt 0 ]; then
            local max_j=$(($i + 1 + score < ${#cards[@]} ? $i + 1 + score : ${#cards[@]}))
            for ((j = i + 1; j < max_j; j++)); do
                counts[$j]=$((${counts[$j]} + ${counts[$i]}))
            done
        fi
    done

    local sum=0
    for count in "${counts[@]}"; do
        ((sum += count))
    done

    echo $sum
}

main() {
    local input_str=$(<input.txt)

    local start=$(date +%s%N)
    local result=$(solve "$input_str")
    echo -e "\nResult: $result"

    local end=$(date +%s%N)
    local time_taken=$(( (end - start) / 1000 ))
    echo "Time taken: ${time_taken} µs"
}

main
