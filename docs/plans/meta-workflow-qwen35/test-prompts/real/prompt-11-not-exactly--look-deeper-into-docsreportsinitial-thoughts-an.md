<!--
Source Session: ses_26c7a2ae6ffeoN2fj7KoPLK03k
Part ID: prt_d93b87023001jXMAJJpeKmAv0g
Character Count: 12137
Extracted: 2026-06-17T07:50:39Z
-->

not exactly.  Look deeper into docs/reports/initial-thoughts and read and aggregate your relevant findings and incorporate this into your understanding and make a new folder ~/code/left-right/language-summary/ .md report document suite file at the top level based on all of the documentation, even those top 3 files I wrote which I never want you to edit (make an opencode doc to enforce this), my initial language documentation goals based on the discussion below, some options for file extension names as I'm sure .lr is already taken, and example IO for the transpile that demonstrates all the language features in it's own sub folder report suite, and a sub suite folder report suite also on cli user flows and desired behavior and what is needed to get there, and thoughts I've had on the topic, and this info:

This a general purpose scripting language that can transpile into js and rust.  It is a fully transpiled language and is meant to used as such similar to purescript, but with a loosely typed point free style that allows for chaining ideas together in line similar to APL, but in a structure that looks and acts similar to json while moving a lot of the usage complexity around certain features to be implicit.  Like your main file is basically just a giant hashmap-arrays of intent but it will have our own semantics about how it works around files and functions and making it interoperable with JS and rust libraries so we aren't trying to reinvent all the libraries, and I want the transpiler written in rust.  when you run `lr` in the cli it opens a TUI shell similar to node but with UI features written in Ink to help with trying things out in the language and editing the semantics of the language easily with the keyboard but still in a visually easy way to navigate with clickable interfaces if that is possible with ink.  I also want `lr "path-to-left-right-file"` to run the transpilation and rust code by default, but can be set in global machine configuration files to transpile and execute in node.  It will also support a watch mode so if you run `lr --watch` it will transpile anything you have in that folder and run it, then listen for file changes, then re-transpile and execute on file change saves. 

Also here is a chat I had about it with another engineer:


Jon Penwood 3:16 PM

You're the only Person I've met who... 😬🙃

Hey!

You might not remember me, as we didn't work together, but we did cross paths a few times probably close to 8 years ago at Daugherty.

I don't think I've met a mind like yours before or since.

Please don't hate me for this, but can you look at this programming language I'm working on 😬, and provide some constructive criticism on the ideas expressed and potential gotchas?

Thinking about posting solutions to RosettaCode or the modern equivalent and could use some eyes from someone who might understand it clearly, and have an interest in programming language aesthetics.

Literally no one in my life understands what point free operator based array oriented programming language even relates to, much less is.

*** Would you be okay with me sending 2 code snippets that demonstrate most of the language features? 

It's 79 lines long, and I think you'll likely understand right away.

It's a JSON oriented, APL/J descendent, minimalist, point free operator based, loosely typed language. 

For real no worries if this is just annoying 👍. I understand sometimes this stuff can be cringe 😬😂
View Jon’s profile Jon Penwood
Jon Penwood 3:24 PM

I'm making it open source, but no desire to bring others in on the work end.

I just honestly have no clue who might look at my art and actually see it. It's really an aesthetic practice for me these last 3 years, but I've kind of been alone with it this whole time, and just would like to make sure I'm not off my rocker and on the right track if you are interested in doing so 👍
Aug 21, 2025
Kieran Brown sent the following message at 8:31 AM
View Kieran’s profile Kieran Brown
Kieran Brown 8:31 AM

I'd love to! Sorry it took so long to get back. I'm always happy to see people pushing minimalist language design. I still feel that's an area that has so much potential!
Jon Penwood sent the following messages at 11:21 AM
View Jon’s profile Jon Penwood
Jon Penwood 11:21 AM

// TS
const getEntityTypes = (
 typesToGet: EntityType | EntityType[],
 entities: Entity[]
): Entity[] => {
 const lowerTypesToGet: string[] =
 typeof typesToGet === 'string' ? [toLower(typesToGet)] : map(toLower, typesToGet);

 const entitiesOfTypesToGet: Entity[] = filter((entity: Entity): boolean => {
 const lowerEntityTypes: string[] = map(toLower, entity.types);

 const entityTypesAreInTypesToGet: boolean = some(
 (typeToGet: string): boolean => lowerEntityTypes.includes(typeToGet),
 lowerTypesToGet
 );

 return entityTypesAreInTypesToGet;
 }, entities);

 return entitiesOfTypesToGet;
};

export default getEntityTypes;

/** PenroScript - file name "getEntityTypes.prsc"
{ typesToGet: _<@0, entities: _<@1,
 lowerTypesToGet: { 
 typesToGet !? = `string`: [typesToGet],
 typesToGet " '_ 
 },

 entityTypesToGet: entities ? {
 lowerEntityTypes: entities@`types` " '_,
 entityTypesAreInTypesToGet: lowerTypesToGet ?| {
 typeToGet: _<@0,
 lowerEntityTypes ?>< typeToGet
 },
 entityTypesAreInTypesToGet
 },

 entityTypesToGet
}
*/

// JS with Lodash FP
({ threats }) => {
 const maliciousThreatsCount = flow(
 filter((threat) => get(`['AI Confidence Level'].value`, threat) === 'malicious'),
 size
 )(threats);

 const threatClassifications = flow(
 map(flow(get(`['Classification'].value`), capitalize)),
 uniq,
 join(', '),
 (threatClassifications) =>
 threatClassifications && `Threat Classifications: ${threatClassifications}`
 )(threats);

 return []
 .concat(maliciousThreatsCount)
 .concat(threatClassifications)
}

/** Penroscript anonymous function 
{ threats: _<@[0,'threats'],
 maliciousThreatsCount: threats 
 $?{ _<@['AI Confidence Level', 'value'] = 'malicious' } 
 #,
 threatClassifications: threats 
 ${ _<@['AI Confidence Level', 'value'] "^_}
 ~ 
 >< ', '
 { threatClassifications: _<, 
 threatClassifications & 'Threat Classifications: {threatClassifications}'
 },

 [] + maliciousThreatsCount + threatClassifications
}
Notes:
- Types: Operator, Hashmap, Array, String, Boolean, Number, Undefined
- Diatic operators are left hungry curried by default, but can be reversed
- Expressions are left to right evaluated, but can be grouped with parentheses
- {... endingNonKeyValueExpression } or {... _< ...} is an operator while {... key: value } is a JSON object
- All strings are template literals from an interface standpoint, but can be expressed as an operator if any the template expressions if the _< or _> operators are used
- Core language operators behavior is input type dependent
- Operators symbols can be overridden and extended
- Symbology is spacial & compounding, so 'asdf' "^ is toUpperCase while 'asdf' "^_ is capitalize"
*/(Edited)
View Jon’s profile Jon Penwood
Jon Penwood 11:23 AM

Thank you for real 🙌! I'm not in a rush, and I'm not under the delusion that people would ever actually use it 😅.

With that in mind, would be fun to have my own running eso-lang.  However, with this level weirdness, I have no clue if this really makes any sense 🥸😅.(Edited)
Kieran Brown sent the following messages at 11:38 AM
View Kieran’s profile Kieran Brown
Kieran Brown 11:38 AM

Can you put that in a google doc and link it there? The chat window is annoyingly small 🤣
View Kieran’s profile Kieran Brown
Kieran Brown 11:42 AM

What's your formal definition for _< and _>
View Kieran’s profile Kieran Brown
Kieran Brown 11:44 AM

Honestly, where's the full syntax definition? 😁
This is interesting in that you're taking a bunch of the more functional world ideas but using symbols in a very algol family kind of way in your syntax. Gives it a unique flavor to read.
Jon Penwood sent the following messages at 3:40 PM
View Jon’s profile Jon Penwood
Jon Penwood 3:40 PM

Sorry about that 😬. Here it is in Docs 
https://docs.google.com/document/d/1WUkQoGcvu3V-zyJIgA9ppDvlKfCGQuNXzimYOtnV5ic/edit?usp=sharing

So funny story 😅,  I worked on this team for 6 years, thought I could trust them, so left all my Language notes and definitions over the last 3ish years on my work laptop not thinking anything of it. I got let go last week out of no where and 10 seconds after the call they lock me out, and they wiped my machine 🫠.  Cybersecurity industry standard practice they say 🤷.

So all of my thoughts, and manually built out ASTs were lost. Luckily I had a coworker I sent this code snippet to, and they got sign off to send me the snippet as they had no interest in the IP 🙌.

This snippet is all I have outside of memory, and I wrote those notes on the bottom from memory. I remember how it all works and flows for the most part, and don't think I'll likely have much issue reconstructing it and writing the parser. 

Excuse my lack of formality in my explanation 🙏

_< or _> if found {} make the Hashmap or String into an unexecuted operator. APL {⍺+⍵} has the same result as {_<+_>}

PenroScript
docs.google.com
View Jon’s profile Jon Penwood
Jon Penwood 3:42 PM

Not sure if you've ever heard the story about the Writing of Dr. Jekyll and Mr Hyde but it's pretty crazy.

The legend I heard was that he wrote it in 3 days, his wife burned it and shoved him down the stairs consensually, and told him to rewrite it from scratch and a few days later he wrote the literary masterpiece we know today!

With this snippet, the simplicity of the language features, and no timeline, I have it a million times easier 🧐🙌

Sad I didn't take it seriously enough to back it up anywhere 😔. Remote jobs can be weird to navigate politically sometimes 😅, especially the first year of an acquisition.(Edited)
Kieran Brown sent the following messages at 4:02 PM
View Kieran’s profile Kieran Brown
Kieran Brown 4:02 PM

I'm interested to know more at least. which says something.

If you're looking to share out a new esolang my fav way is when people make "by example" docs/websites IE https://kimh.github.io/clojure-by-example/#about-this-page

So maybe you should have a little fun making one for yourself ^_^

About This Page
kimh.github.io
I don't like reading thick O'Reilly books when I start learning new programming languages. Rather, I like starting by writing small and dirty code. If you take this approach, having many simple code examples are extremely helpful because I can find...

I do think your focus on "A symbol for the core things we do" is unusual, and I'd need to see more before I love it or hate it.

Jon Penwood sent the following messages at 4:10 PM
View Jon’s profile Jon Penwood
Jon Penwood 4:10 PM

This message has been deleted.
View Jon’s profile Jon Penwood
Jon Penwood 4:12 PM
Kieran: Honestly, where's the full syntax definition? 😁 This is interesting in that you're taking a bunch of the more functional world ideas but using symbols in a very algol family kind of way in your syntax. Gives it a unique flavor to read.

I can definitely see the resemblance to Modula‑2 & Oberon in terms of the algol family 👀

My goal is to create the benefits of Array Oriented Languages, like APL, BQN, & J, but in the loosely typed declarative point free FP world in the Web Dev domain. Lots of inspiration from J honestly. Tried to encode my thought process change in my TS and JS code after learning APL then J. Trying to minimize the language features of J significantly, then adapt it to web dev.
View Jon’s profile Jon Penwood
Jon Penwood 4:20 PM
Kieran: I'm interested to know more at least. which says something. If you're looking to share out a new esolang my fav way is when people make "by example" docs/websites IE https://kimh.github.io/clojure-by-example/#about-this-page So maybe you should have a little fun making one for yourself ^_^

    👏
    👍
    😊

Will do! Seriously appreciate the map 🪧 🗺️ 🙌!

 Haven't used closure since my Daugherty days 🤔.  Maybe I'll write the transpiler in closure from PenroScript to JS and try to make it isomorphic, that way I can plug into npm 🙃.
(Edited)
