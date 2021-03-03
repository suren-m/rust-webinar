using System;
using System.Collections.Generic;

namespace object_passing
{
    class Program
    {
        class Person {
           public string Name { get; set; }
           public int Id { get; set;}
        }

        static void Main(string[] args)
        {
            string s = "Bob";
            get_len(s);
            Console.WriteLine($"String used: {s}\n");

            int get_len(string s){
                return s.Length;
            }

            string s2 = "Bob"; // or object
            change_string(s2);
            Console.WriteLine($"String after func call: {s2}\n");

            void change_string(string s) {
                s = "Alice";    
                Console.WriteLine($"String inside func: {s}");
            }

            Person p = new Person { Name = "Bob", Id = 1};
            change_person(p);
            Console.WriteLine($"Person after func call: {p.Name}, {p.Id}\n");

            void change_person(Person p) {
                p.Name = "Alice";   
                p.Id = 100; 
                Console.WriteLine($"person inside func: {p.Name}, {p.Id}");
            }

            var items = new List<int> { 1, 2, 3};     
            change_list(items);      
            Console.WriteLine("List after func call:");
            items.ForEach(Console.WriteLine);

            void change_list(List<int> items) {
               items[1] = 20; 
               Console.WriteLine("List inside func:");
               items.ForEach(Console.WriteLine);
            }
        }
    }
}
