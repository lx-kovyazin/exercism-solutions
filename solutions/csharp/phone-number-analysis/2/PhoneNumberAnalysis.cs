using System;
using System.Linq;

public static class PhoneNumber
{
    public static (bool IsNewYork, bool IsFake, string LocalNumber) Analyze(string phoneNumber) =>
        phoneNumber.Split('-') is [var maybeNy, var maybeFake, var local]
            ? (maybeNy is "212", maybeFake is "555", local)
            : default;
    
    public static bool IsFake((bool IsNewYork, bool IsFake, string LocalNumber) phoneNumberInfo) => phoneNumberInfo.IsFake;
}
