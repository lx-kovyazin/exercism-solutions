public static class DifferenceOfSquares
{
    static int Square(int x) => x * x;

    static int SumBy(int count, Func<int, int>? mapper = null, int from = 1) =>
        Enumerable.Range(from, int.Max(0, count)).Sum(mapper ?? (static x => x));
    
    public static int CalculateSquareOfSum(int max) =>
        Square(SumBy(max));

    public static int CalculateSumOfSquares(int max) =>
        SumBy(max, Square);

    public static int CalculateDifferenceOfSquares(int max) =>
        CalculateSquareOfSum(max) - CalculateSumOfSquares(max);
}