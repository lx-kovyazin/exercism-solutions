using System.Collections.Generic;
using System.Linq;
using System.Text;

public static class Identifier
{
    public static string Clean(string identifier)
    {
        StringBuilder sb = new();
        bool toUpperIsRequired = false;
        foreach (var c in identifier)
        {
            switch (c)
            {
                case '-':
                    toUpperIsRequired = true;
                    break;

                case ' ':
                    sb.Append('_');
                    break;

                case var _ when char.IsControl(c):
                    sb.Append("CTRL");
                    break;

                case var x when char.IsLetter(x) && !char.IsBetween(x, 'α', 'ω'):
                    if (toUpperIsRequired)
                    {
                        x = char.ToUpper(x);
                        toUpperIsRequired = false;
                    }
                    sb.Append(x);
                    break;
            }
        }
        return sb.ToString();
    }
}
