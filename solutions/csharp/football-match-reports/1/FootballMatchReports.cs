using System;

public static class PlayAnalyzer
{
    public static string AnalyzeOnField(int shirtNum)
        => shirtNum switch
        {
            1 => "goalie",
            2 => "left back",
            5 => "right back",
            9 => "left wing",
            10 => "striker",
            11 => "right wing",
            3 or 4 => "center back",
            6 or 7 or 8 => "midfielder",
            _ => throw new ArgumentOutOfRangeException("An unknown shirt number.")
        };

    public static string AnalyzeOffField(object report)
        => report switch {
            int x => $"There are {x} supporters at the match.",
            string s => s,
            Incident i => i is Injury
                ? $"Oh no! {i.GetDescription()} Medics are on the field."
                : i.GetDescription(),
            Manager m => m.Name + (m.Club is null ? string.Empty : $" ({m.Club})"),
            _ => throw new ArgumentException("An unknown type.")
        };
}
