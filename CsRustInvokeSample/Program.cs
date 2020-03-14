using System;
using System.Runtime.InteropServices;

namespace CsRustInvokeSample
{
    class Program
    {

        static void Main(string[] args)
        {
            using (var rs = new RustSample())
            {
                for (int i = 0; i < 5; i++)
                {
                    rs.Add(10);
                    Console.WriteLine(rs.GetCurrentValue());
                }

                Console.WriteLine("Length = {0}", rs.AppendChars("Hello,"));
                Console.WriteLine("Length = {0}", rs.AppendChars("RustSample!"));
                rs.PrintChars();
            }
        }
    }
}
