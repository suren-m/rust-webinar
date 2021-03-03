using System;
using System.Collections.Generic;

namespace object_passing
{
    class Person {
        public string Name { get; set; }
    }
    class Program
    {
        static void Main(string[] args)
        {
            string s = "Bob"; // or object
            do_something(s);
            Console.WriteLine($"String after func call: {s}\n");

            Person p = new Person { Name = "Bob"};
            do_something(p);
            Console.WriteLine($"Person after func call: {p.Name}\n");

            var items = new List<int> { 1, 2, 3};     
            do_something(items);      
            Console.WriteLine("List after func call:");
            items.ForEach(Console.WriteLine);
        }

        private static void do_something(string s) {
            s = "Alice";    
            Console.WriteLine($"String inside func: {s}");
        }

        private static void do_something(Person p) {
            p.Name = "Alice";    
            Console.WriteLine($"person inside func: {p.Name}");
        }

        private static void do_something(List<int> items) {
            items[1] = 20; 
            Console.WriteLine("List inside func:");
            items.ForEach(Console.WriteLine);
        }

    }
}
