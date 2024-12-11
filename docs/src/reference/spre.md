A Spatial Regular Expression (SpRE) is a spatial- and temporal-based querying language that describes a perception scenario of interest. This querying language is based on traditional Regular Expression patterns found in popular tools such as [grep](https://www.gnu.org/software/grep/manual/grep.html).

!!! example

	Find two to five frames where a car and pedestrian are present.

    ```
	[[:car:] & [:pedestrian:]]{2,5}
	```

## Grammar

The grammar below provides a method for developing valid SpRE patterns.

```
<spre>   ::= '(' <spre> ')'
         | <spre> '*'
         | <spre> <spre>
         | <spre> '|' <spre>
         | <spre> <range>
         | '[' <s4u> ']'
	   
<s4u>    ::= '(' <s4u> ')'
         | <s4u> '&' <s4u>
         | <s4u> '|' <s4u>
         | 'NE' <class>
         | 'NE' '(' <s4> ')'
         | <s4m> '<' <s4m>
         | <s4m> '>' <s4m>
         | <s4m> '<=' <s4m>
         | <s4m> '>=' <s4m>
         | <class>

<s4m>    ::= '(' <s4m> ')'
         | Real 
         | Integer 
         | '@' ('dist' | 'x' | 'y' | 'area') '(' <s4> ')'
         | '@' 'dist' '(' <s4> ',' <s4> ')' 
         | '-' <s4m>
         | <s4m> '-' <s4m> 
         | <s4m> '*' <s4m> 
         | <s4m> '/' <s4m>
    
<s4>     ::= '(' <s4> ')'
         | <s4> '&' <s4>
         | <s4> '|' <s4>
         | '!' <s4>
         | <class>

<class>  ::= <object>

<object> ::= '[' ':' <string> ':' ']'

<range>  ::= '{' <integer> '}'
         | '{' <integer> ',' '}'
         | '{' <integer> ',' <integer> '}'
```
