using System;

public static class SimpleCalculator
{
    public static string Calculate(int operand1, int operand2, string operation)
    {
        try
        {
            switch (operation)
            {
                case "+":
                    return $"{operand1} + {operand2} = {operand1 + operand2}";
                case "*":
                    return $"{operand1} * {operand2} = {operand1 * operand2}";
                case "/":
                    return $"{operand1} / {operand2} = {operand1 / operand2}";

                case "": throw new ArgumentException();
                case null: throw new ArgumentNullException();
                default: throw new ArgumentOutOfRangeException();
            };
        }
        catch (DivideByZeroException)
        {
            return "Division by zero is not allowed.";
        }
    }
}
