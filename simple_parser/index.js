const class_regex = /class\s*=\s*"[\w-\s]*"/g;
const { every } = require("underscore");

/**
 * @param {SimpleParser} parser_obj - The parser object
 * @param {number} start - Start index
 * @param {number} end - End index
 */
class SimpleNode {
    constructor(parser_obj, start, end) {
        this.parser = parser_obj;
        this.start = start;
        this.end = end;
        this.children = [];

        const classes = [];

        const tag_name = '';
        const string = this.parser.html.substr(start, end - start); // didn't want to create so many strings in recursion, but there's no string_view here, and regex needs it 😢

        if( !/^\s*</.test(string) ) {
            throw new Error(`Parsing Error: Between ${start} - ${end}; Explain: Not valid HTML tag, doesn't start with '<'`);
        }

        const reg = /<\w>/
    }

    to_string() {
        return this.parser.html.substr(this.start, this.end);
    }
}

class SimpleParser {
    constructor(html_string) {
        if(typeof html_string !== 'string') {
            throw new Error("HTML should be string");
        }

        this.html = html_string;
        if (typeof html_string !== 'string') return;
        this.class_map = new Map();

        console.log("Received length: ", html_string.length);

        // TODO - To support all, this needs to be improved further

        console.log(typeof this);
        this.root = new SimpleNode(this, 0, html_string.length);
        // let count = 0;
        // const matches = html_string.matchAll(class_regex);
        // for (const match of matches) {
        //     const matched = match['0'];
        //     const start = match.index;
        //     const end = start + match['0'].length;
        //     // console.log(match.index, match[0].length, match[0], match.input.length, match.groups);
        //     // console.log(Object.keys(match));
        //     ++count;
        //     if (count > 20) break;
        // }
    }

    selectOneByClassName(selectors) {
        return this.selectAllByClassName(selectors)[0] || null;
    }

    selectAllByClassName(selectors) {
        if (!Array.isArray(selectors)) {
            if (typeof selectors !== 'string') return [];

            selectors = selectors.split(/ +/);    // split using regex, so multiple spaces ignored too
        }

        const matches = (selectors.length === 0) || !this.class_map.has(selectors[0]) ? [] : this.class_map.get(selectors[0]);

        matches.filter((val) => {
            return every(
                selectors,
                css_style => (
                    this.class_map.has(css_style)
                    &&
                    this.class_map.get(css_style).includes(val)
                )
            );
        })

        return matches;
    }
}

module.exports = SimpleParser;
