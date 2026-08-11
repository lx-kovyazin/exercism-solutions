public static class DifferenceOfSquares
{
    static IEnumerable<int> GenerateSeq(int max) =>
        Enumerable.Range(1, max < 0 ? 0 : max);

    static int Square(int x) => x * x;
    
    public static int CalculateSquareOfSum(int max) =>
        Square(GenerateSeq(max).Sum());

    public static int CalculateSumOfSquares(int max) =>
        GenerateSeq(max).Sum(Square);

    public static int CalculateDifferenceOfSquares(int max) =>
        CalculateSquareOfSum(max) - CalculateSumOfSquares(max);
}