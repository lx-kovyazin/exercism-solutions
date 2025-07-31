using System;
using System.Collections.Generic;
using System.Linq;

public static class Languages
{
    public static List<string> NewList() => new();

    public static List<string> GetExistingLanguages()
        => new() { "C#", "Clojure", "Elm" };

    public static List<string> AddLanguage(List<string> languages, string language)
        => languages.Append(language).ToList();

    public static int CountLanguages(List<string> languages)
        => languages.Count;
    public static bool HasLanguage(List<string> languages, string language)
        => languages.Contains(language);

    public static List<string> ReverseList(List<string> languages)
        => languages.AsEnumerable().Reverse().ToList();

    public static bool IsExciting(List<string> languages)
        => languages.Count is > 0 && languages.First().Equals("C#")
        || languages.Count is 2 or 3 && languages.Skip(1).First().Equals("C#");

    public static List<string> RemoveLanguage(List<string> languages, string language)
    {
        languages.Remove(language);
        return languages;
    }

    public static bool IsUnique(List<string> languages)
        => languages.Count == languages.Distinct().Count();
}
