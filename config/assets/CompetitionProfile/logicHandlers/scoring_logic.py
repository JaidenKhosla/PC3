def scoring(problem_name: str, initial_problem_score: int, nth_attempt: int, time: int) -> int:
    return (initial_problem_score - 2 * min(nth_attempt, 1))