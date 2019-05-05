////////////////////////////////////////////////////////////////////////////////
// TinyBasic Plus
////////////////////////////////////////////////////////////////////////////////
//
// Authors:
//    Gordon Brandly (Tiny Basic for 68000)
//    Mike Field <hamster@snap.net.nz> (Arduino Basic) (port to Arduino)
//    Scott Lawrence <yorgle@gmail.com> (TinyBasic Plus) (features, etc)
//
// Contributors:
//          Brian O'Dell <megamemnon@megamemnon.com> (INPUT)
//    (full list tbd)

//  For full history of Tiny Basic, please see the wikipedia entry here:
//    https://en.wikipedia.org/wiki/Tiny_BASIC

// LICENSING NOTES:
//    Mike Field based his C port of Tiny Basic on the 68000
//    Tiny BASIC which carried the following license:
/*
******************************************************************
*                                                                *
*               Tiny BASIC for the Motorola MC68000              *
*                                                                *
* Derived from Palo Alto Tiny BASIC as published in the May 1976 *
* issue of Dr. Dobb's Journal.  Adapted to the 68000 by:         *
*       Gordon Brandly                                           *
*       12147 - 51 Street                                        *
*       Edmonton AB  T5W 3G8                                     *
*       Canada                                                   *
*       (updated mailing address for 1996)                       *
*                                                                *
* This version is for MEX68KECB Educational Computer Board I/O.  *
*                                                                *
******************************************************************
*    Copyright (C) 1984 by Gordon Brandly. This program may be   *
*    freely distributed for personal use only. All commercial    *
*                      rights are reserved.                      *
******************************************************************
*/
//    ref: http://members.shaw.ca:80/gbrandly/68ktinyb.html
//
//    However, Mike did not include a license of his own for his
//    version of this.
//    ref: http://hamsterworks.co.nz/mediawiki/index.php/Arduino_Basic
//
//    From discussions with him, I felt that the MIT license is
//    the most applicable to his intent.
//
//    I am in the process of further determining what should be
//    done wrt licensing further.  This entire header will likely
//    change with the next version 0.16, which will hopefully nail
//    down the whole thing so we can get back to implementing
//    features instead of licenses.  Thank you for your time.

#define kVersion "v1.0"

// v1.0: 2018-09-16
//      Ported to Monotron. Removed Arduino support.

// v0.15: 2018-06-23
//      Integrating some contributions
//      Corrected some of the #ifdef nesting atop this page
//      Licensing issues beginning to be addressed

// v0.14: 2013-11-07
//      Modified Input command to accept an expression using getn()
//      Syntax is "input x" where x is any variable
//      NOTE: This only works for numbers, expressions. not strings.
//
// v0.13: 2013-03-04
//      Support for Arduino 1.5 (SPI.h included, additional changes for DUE support)
//
// v0.12: 2013-03-01
//      EEPROM load and save routines added: EFORMAT, ELIST, ELOAD, ESAVE, ECHAIN
//      added EAUTORUN option (chains to EEProm saved program on startup)
//      Bugfixes to build properly on non-arduino systems (PROGMEM #define workaround)
//      cleaned up a bit of the #define options wrt TONE
//
// v0.11: 2013-02-20
//      all display strings and tables moved to PROGMEM to save space
//      removed second serial
//      removed pinMode completely, autoconf is explicit
//      beginnings of EEPROM related functionality (new,load,save,list)
//
// v0.10: 2012-10-15
//      added kAutoConf, which eliminates the "PINMODE" statement.
//      now, DWRITE,DREAD,AWRITE,AREAD automatically set the PINMODE appropriately themselves.
//      should save a few bytes in your programs.
//
// v0.09: 2012-10-12
//      Fixed directory listings.  FILES now always works. (bug in the SD library)
//      ref: http://arduino.cc/forum/index.php/topic,124739.0.html
//      fixed filesize printouts (added printUnum for unsigned numbers)
//      #defineable baud rate for slow connection throttling
//e
// v0.08: 2012-10-02
//      Tone generation through piezo added (TONE, TONEW, NOTONE)
//
// v0.07: 2012-09-30
//      Autorun buildtime configuration feature
//
// v0.06: 2012-09-27
//      Added optional second serial input, used for an external keyboard
//
// v0.05: 2012-09-21
//      CHAIN to load and run a second file
//      RND,RSEED for random stuff
//      Added "!=" for "<>" synonym
//      Added "END" for "STOP" synonym (proper name for the functionality anyway)
//
// v0.04: 2012-09-20
//      DELAY ms   - for delaying
//      PINMODE <pin>, INPUT|IN|I|OUTPUT|OUT|O
//      DWRITE <pin>, HIGH|HI|1|LOW|LO|0
//      AWRITE <pin>, [0..255]
//      fixed "save" appending to existing files instead of overwriting
// 	Updated for building desktop command line app (incomplete)
//
// v0.03: 2012-09-19
//	Integrated Jurg Wullschleger whitespace,unary fix
//	Now available through github
//	Project renamed from "Tiny Basic in C" to "TinyBasic Plus"
//
// v0.02b: 2012-09-17  Scott Lawrence <yorgle@gmail.com>
// 	Better FILES listings
//
// v0.02a: 2012-09-17  Scott Lawrence <yorgle@gmail.com>
// 	Support for SD Library
// 	Added: SAVE, FILES (mostly works), LOAD (mostly works) (redirects IO)
// 	Added: MEM, ? (PRINT)
// 	Quirk:  "10 LET A=B+C" is ok "10 LET A = B + C" is not.
// 	Quirk:  INPUT seems broken?

// IF testing with Visual C, this needs to be the first thing in the file.
//#include "stdafx.h"

#include <stdbool.h>
#include <monotron.h>
#include <string.h>

// size of our program ram
#define kRamSize   19*1024 /* All that fits in Monotron's 24 KiB usable SRAM */

////////////////////

// Required for ARM devices
// #define ALIGN_MEMORY
#ifdef ALIGN_MEMORY
#define ALIGN_UP(x) ((unsigned char*)(((intptr_t)((x) + 3) >> 1) << 1))
#define ALIGN_DOWN(x) ((unsigned char*)(((intptr_t)(x) >> 1) << 1))
#else
#define ALIGN_UP(x) (x)
#define ALIGN_DOWN(x) (x)
#endif

////////////////////

#ifndef byte
typedef unsigned char byte;
#endif

////////////////////

static bool triggerRun = false;

////////////////////////////////////////////////////////////////////////////////
// ASCII Characters
#define CR	'\r'
#define NL	'\n'
#define LF      0x0a
#define TAB	'\t'
#define BELL	'\b'
#define SPACE   ' '
#define SQUOTE  '\''
#define DQUOTE  '\"'
#define CTRLC	0x03
#define CTRLH	0x08
#define CTRLS	0x13
#define CTRLX	0x18

typedef short unsigned LINENUM;

static unsigned char program[kRamSize];
static unsigned char *txtpos,*list_line, *tmptxtpos;
static unsigned char expression_error;
static unsigned char *tempsp;

/***********************************************************/
// Keyword table and constants - the last character has 0x80 added to it
const static unsigned char keywords[]  = {
    'L','I','S','T'+0x80,
    'L','O','A','D'+0x80,
    'N','E','W'+0x80,
    'R','U','N'+0x80,
    'N','E','X','T'+0x80,
    'L','E','T'+0x80,
    'I','F'+0x80,
    'G','O','T','O'+0x80,
    'G','O','S','U','B'+0x80,
    'R','E','T','U','R','N'+0x80,
    'R','E','M'+0x80,
    'F','O','R'+0x80,
    'I','N','P','U','T'+0x80,
    'P','R','I','N','T'+0x80,
    'P','O','K','E'+0x80,
    'S','T','O','P'+0x80,
    'B','Y','E'+0x80,
    'M','E','M'+0x80,
    'R','S','E','E','D'+0x80,
    '?'+ 0x80,
    '\''+ 0x80,
    'E','N','D'+0x80,
    0
};

// by moving the command list to an enum, we can easily remove sections
// above and below simultaneously to selectively obliterate functionality.
enum {
    KW_LIST = 0,
    KW_LOAD,
    KW_NEW,
    KW_RUN,
    KW_NEXT,
    KW_LET,
    KW_IF,
    KW_GOTO,
    KW_GOSUB,
    KW_RETURN,
    KW_REM,
    KW_FOR,
    KW_INPUT,
    KW_PRINT,
    KW_POKE,
    KW_STOP,
    KW_BYE,
    KW_MEM,
    KW_RSEED,
    KW_QMARK,
    KW_QUOTE,
    KW_END,
    KW_DEFAULT /* always the final one*/
};

struct stack_for_frame {
    char frame_type;
    char for_var;
    short int terminal;
    short int step;
    unsigned char *current_line;
    unsigned char *txtpos;
};

struct stack_gosub_frame {
    char frame_type;
    unsigned char *current_line;
    unsigned char *txtpos;
};

const static unsigned char func_tab[]  = {
    'P','E','E','K'+0x80,
    'A','B','S'+0x80,
    'A','R','E','A','D'+0x80,
    'D','R','E','A','D'+0x80,
    'R','N','D'+0x80,
    0
};
#define FUNC_PEEK    0
#define FUNC_ABS     1
#define FUNC_AREAD   2
#define FUNC_DREAD   3
#define FUNC_RND     4
#define FUNC_UNKNOWN 5

const static unsigned char to_tab[]  = {
    'T','O'+0x80,
    0
};

const static unsigned char step_tab[]  = {
    'S','T','E','P'+0x80,
    0
};

const static unsigned char relop_tab[]  = {
    '>','='+0x80,
    '<','>'+0x80,
    '>'+0x80,
    '='+0x80,
    '<','='+0x80,
    '<'+0x80,
    '!','='+0x80,
    0
};

#define RELOP_GE		0
#define RELOP_NE		1
#define RELOP_GT		2
#define RELOP_EQ		3
#define RELOP_LE		4
#define RELOP_LT		5
#define RELOP_NE_BANG	6
#define RELOP_UNKNOWN	7

#define STACK_SIZE (sizeof(struct stack_for_frame)*5)
#define VAR_SIZE sizeof(short int) // Size of variables in bytes

static unsigned char *stack_limit;
static unsigned char *program_start;
static unsigned char *program_end;
static unsigned char *variables_begin;
static unsigned char *current_line;
static unsigned char *sp;
#define STACK_GOSUB_FLAG 'G'
#define STACK_FOR_FLAG 'F'
static unsigned char table_index;
static LINENUM linenum;

static const unsigned char okmsg[]             = "OK";
static const unsigned char whatmsg[]           = "What? ";
static const unsigned char howmsg[]            = "How?";
static const unsigned char sorrymsg[]          = "Sorry!";
static const unsigned char initmsg[]           = "TinyBasic Plus for Monotron " kVersion;
static const unsigned char memorymsg[]         = " bytes free.";
static const unsigned char breakmsg[]          = "break!";
static const unsigned char backspacemsg[]      = "\b \b";

static void ignore_blanks(void);
static void scantable(const unsigned char *table);
static void pushb(unsigned char b);
static unsigned char popb();
static void printnum(int num);
static unsigned short testnum(void);
static unsigned char print_quoted_string(void);
static void printmsgNoNL(const unsigned char *msg);
static void printmsg(const unsigned char *msg);
static void getln(char prompt);
static unsigned char *findline(void);
static void toUppercaseBuffer(void);
static void printline();
static short int expr4(void);
static short int expr3(void);
static short int expr2(void);
static short int expression(void);
static void line_terminator(void);
static unsigned char breakcheck(void);
static int inchar();

/***************************************************************************/
static void ignore_blanks(void) {
    while(*txtpos == SPACE || *txtpos == TAB)
        txtpos++;
}


/***************************************************************************/
static void scantable(const unsigned char *table) {
    int i = 0;
    table_index = 0;
    while(1) {
        // Run out of table entries?
        if(*table == 0)
            return;

        // Do we match this character?
        if(txtpos[i] == *table) {
            i++;
            table++;
        } else {
            // do we match the last character of keywork (with 0x80 added)? If so, return
            if(txtpos[i]+0x80 == *table) {
                txtpos += i+1;  // Advance the pointer to following the keyword
                ignore_blanks();
                return;
            }

            // Forward to the end of this keyword
            while((*table & 0x80) == 0)
                table++;

            // Now move on to the first character of the next word, and reset the position index
            table++;
            table_index++;
            ignore_blanks();
            i = 0;
        }
    }
}

/***************************************************************************/
static void pushb(unsigned char b) {
    sp--;
    *sp = b;
}

/***************************************************************************/
static unsigned char popb() {
    unsigned char b;
    b = *sp;
    sp++;
    return b;
}

/***************************************************************************/
static void printnum(int num) {
    int digits = 0;

    if(num < 0) {
        num = -num;
        putchar('-');
    }
    do {
        pushb(num%10+'0');
        num = num/10;
        digits++;
    } while (num > 0);

    while(digits > 0) {
        putchar(popb());
        digits--;
    }
}

void printUnum(unsigned int num) {
    int digits = 0;

    do {
        pushb(num%10+'0');
        num = num/10;
        digits++;
    } while (num > 0);

    while(digits > 0) {
        putchar(popb());
        digits--;
    }
}

/***************************************************************************/
static unsigned short testnum(void) {
    unsigned short num = 0;
    ignore_blanks();

    while(*txtpos>= '0' && *txtpos <= '9' ) {
        // Trap overflows
        if(num >= 0xFFFF/10) {
            num = 0xFFFF;
            break;
        }

        num = num *10 + *txtpos - '0';
        txtpos++;
    }
    return	num;
}

/***************************************************************************/
static unsigned char print_quoted_string(void) {
    int i=0;
    unsigned char delim = *txtpos;
    if(delim != '"' && delim != '\'')
        return 0;
    txtpos++;

    // Check we have a closing delimiter
    while(txtpos[i] != delim) {
        if(txtpos[i] == NL)
            return 0;
        i++;
    }

    // Print the characters
    while(*txtpos != delim) {
        putchar(*txtpos);
        txtpos++;
    }
    txtpos++; // Skip over the last delimiter

    return 1;
}


/***************************************************************************/
static void printmsgNoNL(const unsigned char *msg) {
    while(*msg != 0) {
        putchar(*msg++);
    }
}

/***************************************************************************/
static void printmsg(const unsigned char *msg) {
    printmsgNoNL(msg);
    line_terminator();
}

/***************************************************************************/
static void getln(char prompt) {
    putchar(prompt);
    txtpos = program_end+sizeof(LINENUM);

    while(1) {
        char c = inchar();
        switch(c) {
        case NL:
        //break;
        case CR:
            line_terminator();
            // Terminate all strings with a NL
            txtpos[0] = NL;
            return;
        case CTRLH:
            if(txtpos == program_end)
                break;
            txtpos--;

            printmsgNoNL(backspacemsg);
            break;
        default:
            // We need to leave at least one space to allow us to shuffle the line into order
            if(txtpos == variables_begin-2)
                putchar(BELL);
            else {
                txtpos[0] = c;
                txtpos++;
                putchar(c);
            }
        }
    }
}

/***************************************************************************/
static unsigned char *findline(void) {
    unsigned char *line = program_start;
    while(1) {
        if(line == program_end)
            return line;

        if(((LINENUM *)line)[0] >= linenum)
            return line;

        // Add the line length onto the current address, to get to the next line;
        line += line[sizeof(LINENUM)];
    }
}

/***************************************************************************/
static void toUppercaseBuffer(void) {
    unsigned char *c = program_end+sizeof(LINENUM);
    unsigned char quote = 0;

    while(*c != NL) {
        // Are we in a quoted string?
        if(*c == quote)
            quote = 0;
        else if(*c == '"' || *c == '\'')
            quote = *c;
        else if(quote == 0 && *c >= 'a' && *c <= 'z')
            *c = *c + 'A' - 'a';
        c++;
    }
}

/***************************************************************************/
static void printline() {
    LINENUM line_num;

    line_num = *((LINENUM *)(list_line));
    list_line += sizeof(LINENUM) + sizeof(char);

    // Output the line */
    printnum(line_num);
    putchar(' ');
    while(*list_line != NL) {
        putchar(*list_line);
        list_line++;
    }
    list_line++;
#ifdef ALIGN_MEMORY
    // Start looking for next line on even page
    if (ALIGN_UP(list_line) != list_line)
        list_line++;
#endif
    line_terminator();
}

/***************************************************************************/
static short int expr4(void) {
    // fix provided by Jurg Wullschleger wullschleger@gmail.com
    // fixes whitespace and unary operations
    ignore_blanks();

    if( *txtpos == '-' ) {
        txtpos++;
        return -expr4();
    }
    // end fix

    if(*txtpos == '0') {
        txtpos++;
        return 0;
    }

    if(*txtpos >= '1' && *txtpos <= '9') {
        short int a = 0;
        do 	{
            a = a*10 + *txtpos - '0';
            txtpos++;
        } while(*txtpos >= '0' && *txtpos <= '9');
        return a;
    }

    // Is it a function or variable reference?
    if(txtpos[0] >= 'A' && txtpos[0] <= 'Z') {
        short int a;
        // Is it a variable reference (single alpha)
        if(txtpos[1] < 'A' || txtpos[1] > 'Z') {
            a = ((short int *)variables_begin)[*txtpos - 'A'];
            txtpos++;
            return a;
        }

        // Is it a function with a single parameter
        scantable(func_tab);
        if(table_index == FUNC_UNKNOWN)
            goto expr4_error;

        unsigned char f = table_index;

        if(*txtpos != '(')
            goto expr4_error;

        txtpos++;
        a = expression();
        if(*txtpos != ')')
            goto expr4_error;
        txtpos++;
        switch(f) {
        case FUNC_PEEK:
            return program[a];

        case FUNC_ABS:
            if(a < 0)
                return -a;
            return a;

        case FUNC_RND:
            return( rand() % a );
        }
    }

    if(*txtpos == '(') {
        short int a;
        txtpos++;
        a = expression();
        if(*txtpos != ')')
            goto expr4_error;

        txtpos++;
        return a;
    }

expr4_error:
    expression_error = 1;
    return 0;

}

/***************************************************************************/
static short int expr3(void) {
    short int a,b;

    a = expr4();

    ignore_blanks(); // fix for eg:  100 a = a + 1

    while(1) {
        if(*txtpos == '*') {
            txtpos++;
            b = expr4();
            a *= b;
        } else if(*txtpos == '%') {
            txtpos++;
            b = expr4();
            if(b != 0)
                a %= b;
            else
                expression_error = 1;
        } else if(*txtpos == '/') {
            txtpos++;
            b = expr4();
            if(b != 0)
                a /= b;
            else
                expression_error = 1;
        } else
            return a;
    }
}

/***************************************************************************/
static short int expr2(void) {
    short int a,b;

    if(*txtpos == '-' || *txtpos == '+')
        a = 0;
    else
        a = expr3();

    while(1) {
        if(*txtpos == '-') {
            txtpos++;
            b = expr3();
            a -= b;
        } else if(*txtpos == '+') {
            txtpos++;
            b = expr3();
            a += b;
        } else
            return a;
    }
}

/***************************************************************************/
static short int expression(void) {
    short int a,b;

    a = expr2();

    // Check if we have an error
    if(expression_error)	return a;

    scantable(relop_tab);
    if(table_index == RELOP_UNKNOWN)
        return a;

    switch(table_index) {
    case RELOP_GE:
        b = expr2();
        if(a >= b) return 1;
        break;
    case RELOP_NE:
    case RELOP_NE_BANG:
        b = expr2();
        if(a != b) return 1;
        break;
    case RELOP_GT:
        b = expr2();
        if(a > b) return 1;
        break;
    case RELOP_EQ:
        b = expr2();
        if(a == b) return 1;
        break;
    case RELOP_LE:
        b = expr2();
        if(a <= b) return 1;
        break;
    case RELOP_LT:
        b = expr2();
        if(a < b) return 1;
        break;
    }
    return 0;
}

/***************************************************************************/
#ifdef LINUX_BUILD
int main(int argc, const char** argv) {
    init();
    int result = monotron_main();
    deinit();
    return result;
}
#endif

/***************************************************************************/
int monotron_main(void) {
    unsigned char *start;
    unsigned char *newEnd;
    unsigned char linelen;

    printmsg(initmsg);

    program_start = program;
    program_end = program_start;
    sp = program+sizeof(program);  // Needed for printnum
#ifdef ALIGN_MEMORY
    // Ensure these memory blocks start on even pages
    stack_limit = ALIGN_DOWN(program+sizeof(program)-STACK_SIZE);
    variables_begin = ALIGN_DOWN(stack_limit - 27*VAR_SIZE);
#else
    stack_limit = program+sizeof(program)-STACK_SIZE;
    variables_begin = stack_limit - 27*VAR_SIZE;
#endif

    // memory free
    printnum(variables_begin-program_end);
    printmsg(memorymsg);

warmstart:
    // this signifies that it is running in 'direct' mode.
    current_line = 0;
    sp = program+sizeof(program);
    printmsg(okmsg);

prompt:
    if( triggerRun ) {
        triggerRun = false;
        current_line = program_start;
        goto execline;
    }

    getln( '>' );
    toUppercaseBuffer();

    txtpos = program_end+sizeof(unsigned short);

    // Find the end of the freshly entered line
    while(*txtpos != NL)
        txtpos++;

    // Move it to the end of program_memory
    {
        unsigned char *dest;
        dest = variables_begin-1;
        while(1) {
            *dest = *txtpos;
            if(txtpos == program_end+sizeof(unsigned short))
                break;
            dest--;
            txtpos--;
        }
        txtpos = dest;
    }

    // Now see if we have a line number
    linenum = testnum();
    ignore_blanks();
    if(linenum == 0)
        goto direct;

    if(linenum == 0xFFFF)
        goto qhow;

    // Find the length of what is left, including the (yet-to-be-populated) line header
    linelen = 0;
    while(txtpos[linelen] != NL)
        linelen++;
    linelen++; // Include the NL in the line length
    linelen += sizeof(unsigned short)+sizeof(char); // Add space for the line number and line length

    // Now we have the number, add the line header.
    txtpos -= 3;

#ifdef ALIGN_MEMORY
    // Line starts should always be on 16-bit pages
    if (ALIGN_DOWN(txtpos) != txtpos) {
        txtpos--;
        linelen++;
        // As the start of the line has moved, the data should move as well
        unsigned char *tomove;
        tomove = txtpos + 3;
        while (tomove < txtpos + linelen - 1) {
            *tomove = *(tomove + 1);
            tomove++;
        }
    }
#endif

    *((unsigned short *)txtpos) = linenum;
    txtpos[sizeof(LINENUM)] = linelen;


    // Merge it into the rest of the program
    start = findline();

    // If a line with that number exists, then remove it
    if(start != program_end && *((LINENUM *)start) == linenum) {
        unsigned char *dest, *from;
        unsigned tomove;

        from = start + start[sizeof(LINENUM)];
        dest = start;

        tomove = program_end - from;
        while( tomove > 0) {
            *dest = *from;
            from++;
            dest++;
            tomove--;
        }
        program_end = dest;
    }

    if(txtpos[sizeof(LINENUM)+sizeof(char)] == NL) // If the line has no txt, it was just a delete
        goto prompt;



    // Make room for the new line, either all in one hit or lots of little shuffles
    while(linelen > 0) {
        unsigned int tomove;
        unsigned char *from,*dest;
        unsigned int space_to_make;

        space_to_make = txtpos - program_end;

        if(space_to_make > linelen)
            space_to_make = linelen;
        newEnd = program_end+space_to_make;
        tomove = program_end - start;


        // Source and destination - as these areas may overlap we need to move bottom up
        from = program_end;
        dest = newEnd;
        while(tomove > 0) {
            from--;
            dest--;
            *dest = *from;
            tomove--;
        }

        // Copy over the bytes into the new space
        for(tomove = 0; tomove < space_to_make; tomove++) {
            *start = *txtpos;
            txtpos++;
            start++;
            linelen--;
        }
        program_end = newEnd;
    }
    goto prompt;

qhow:
    printmsg(howmsg);
    goto prompt;

qwhat:
    printmsgNoNL(whatmsg);
    if(current_line != NULL) {
        unsigned char tmp = *txtpos;
        if(*txtpos != NL)
            *txtpos = '^';
        list_line = current_line;
        printline();
        *txtpos = tmp;
    }
    line_terminator();
    goto prompt;

qsorry:
    printmsg(sorrymsg);
    goto warmstart;

run_next_statement:
    while(*txtpos == ':')
        txtpos++;
    ignore_blanks();
    if(*txtpos == NL)
        goto execnextline;
    goto interperateAtTxtpos;

direct:
    txtpos = program_end+sizeof(LINENUM);
    if(*txtpos == NL)
        goto prompt;

interperateAtTxtpos:
    if(breakcheck()) {
        printmsg(breakmsg);
        goto warmstart;
    }


    scantable(keywords);

    switch(table_index) {
    case KW_LIST:
        goto list;
    case KW_MEM:
        goto mem;
    case KW_NEW:
        if(txtpos[0] != NL)
            goto qwhat;
        program_end = program_start;
        goto prompt;
    case KW_RUN:
        current_line = program_start;
        goto execline;
    case KW_NEXT:
        goto next;
    case KW_LET:
        goto assignment;
    case KW_IF:
        {
            short int val;
            expression_error = 0;
            val = expression();
            if(expression_error || *txtpos == NL)
                goto qhow;
            if(val != 0)
                goto interperateAtTxtpos;
            goto execnextline;
        }
    case KW_GOTO:
        expression_error = 0;
        linenum = expression();
        if(expression_error || *txtpos != NL)
            goto qhow;
        current_line = findline();
        goto execline;
    case KW_GOSUB:
        goto gosub;
    case KW_RETURN:
        goto gosub_return;
    case KW_REM:
    case KW_QUOTE:
        goto execnextline;	// Ignore line completely
    case KW_FOR:
        goto forloop;
    case KW_INPUT:
        goto input;
    case KW_PRINT:
    case KW_QMARK:
        goto print;
    case KW_POKE:
        goto poke;
    case KW_END:
    case KW_STOP:
        // This is the easy way to end - set the current line to the end of program attempt to run it
        if(txtpos[0] != NL)
            goto qwhat;
        current_line = program_end;
        goto execline;
    case KW_BYE:
        // Leave the basic interperater
        return 0;
    case KW_RSEED:
        goto rseed;
    case KW_DEFAULT:
        goto assignment;
    default:
        break;
    }

execnextline:
    if(current_line == NULL)		// Processing direct commands?
        goto prompt;
    current_line +=	 current_line[sizeof(LINENUM)];

execline:
    if(current_line == program_end) // Out of lines to run
        goto warmstart;
    txtpos = current_line+sizeof(LINENUM)+sizeof(char);
    goto interperateAtTxtpos;

input: {
        unsigned char var;
        int value;
        ignore_blanks();
        if(*txtpos < 'A' || *txtpos > 'Z')
            goto qwhat;
        var = *txtpos;
        txtpos++;
        ignore_blanks();
        if(*txtpos != NL && *txtpos != ':')
            goto qwhat;
inputagain:
        tmptxtpos = txtpos;
        getln( '?' );
        toUppercaseBuffer();
        txtpos = program_end+sizeof(unsigned short);
        ignore_blanks();
        expression_error = 0;
        value = expression();
        if(expression_error)
            goto inputagain;
        ((short int *)variables_begin)[var-'A'] = value;
        txtpos = tmptxtpos;

        goto run_next_statement;
    }

forloop: {
        unsigned char var;
        short int initial, step, terminal;
        ignore_blanks();
        if(*txtpos < 'A' || *txtpos > 'Z')
            goto qwhat;
        var = *txtpos;
        txtpos++;
        ignore_blanks();
        if(*txtpos != '=')
            goto qwhat;
        txtpos++;
        ignore_blanks();

        expression_error = 0;
        initial = expression();
        if(expression_error)
            goto qwhat;

        scantable(to_tab);
        if(table_index != 0)
            goto qwhat;

        terminal = expression();
        if(expression_error)
            goto qwhat;

        scantable(step_tab);
        if(table_index == 0) {
            step = expression();
            if(expression_error)
                goto qwhat;
        } else
            step = 1;
        ignore_blanks();
        if(*txtpos != NL && *txtpos != ':')
            goto qwhat;



        if(!expression_error && *txtpos == NL) {
            struct stack_for_frame *f;
            if(sp + sizeof(struct stack_for_frame) < stack_limit)
                goto qsorry;

            sp -= sizeof(struct stack_for_frame);
            f = (struct stack_for_frame *)sp;
            ((short int *)variables_begin)[var-'A'] = initial;
            f->frame_type = STACK_FOR_FLAG;
            f->for_var = var;
            f->terminal = terminal;
            f->step     = step;
            f->txtpos   = txtpos;
            f->current_line = current_line;
            goto run_next_statement;
        }
    }
    goto qhow;

gosub:
    expression_error = 0;
    linenum = expression();
    if(!expression_error && *txtpos == NL) {
        struct stack_gosub_frame *f;
        if(sp + sizeof(struct stack_gosub_frame) < stack_limit)
            goto qsorry;

        sp -= sizeof(struct stack_gosub_frame);
        f = (struct stack_gosub_frame *)sp;
        f->frame_type = STACK_GOSUB_FLAG;
        f->txtpos = txtpos;
        f->current_line = current_line;
        current_line = findline();
        goto execline;
    }
    goto qhow;

next:
    // Fnd the variable name
    ignore_blanks();
    if(*txtpos < 'A' || *txtpos > 'Z')
        goto qhow;
    txtpos++;
    ignore_blanks();
    if(*txtpos != ':' && *txtpos != NL)
        goto qwhat;

gosub_return:
    // Now walk up the stack frames and find the frame we want, if present
    tempsp = sp;
    while(tempsp < program+sizeof(program)-1) {
        switch(tempsp[0]) {
        case STACK_GOSUB_FLAG:
            if(table_index == KW_RETURN) {
                struct stack_gosub_frame *f = (struct stack_gosub_frame *)tempsp;
                current_line    = f->current_line;
                txtpos          = f->txtpos;
                sp += sizeof(struct stack_gosub_frame);
                goto run_next_statement;
            }
            // This is not the loop you are looking for... so Walk back up the stack
            tempsp += sizeof(struct stack_gosub_frame);
            break;
        case STACK_FOR_FLAG:
            // Flag, Var, Final, Step
            if(table_index == KW_NEXT) {
                struct stack_for_frame *f = (struct stack_for_frame *)tempsp;
                // Is the the variable we are looking for?
                if(txtpos[-1] == f->for_var) {
                    short int *varaddr = ((short int *)variables_begin) + txtpos[-1] - 'A';
                    *varaddr = *varaddr + f->step;
                    // Use a different test depending on the sign of the step increment
                    if((f->step > 0 && *varaddr <= f->terminal) || (f->step < 0 && *varaddr >= f->terminal)) {
                        // We have to loop so don't pop the stack
                        txtpos = f->txtpos;
                        current_line = f->current_line;
                        sp = tempsp;
                        goto run_next_statement;
                    }
                    // We've run to the end of the loop. drop out of the loop, popping the stack
                    sp = tempsp + sizeof(struct stack_for_frame);
                    goto run_next_statement;
                }
            }
            // This is not the loop you are looking for... so Walk back up the stack
            tempsp += sizeof(struct stack_for_frame);
            break;
        default:
            goto warmstart;
        }
    }
    // Didn't find the variable we've been looking for
    goto qhow;

assignment: {
        short int value;
        short int *var;

        if(*txtpos < 'A' || *txtpos > 'Z')
            goto qhow;
        var = (short int *)variables_begin + *txtpos - 'A';
        txtpos++;

        ignore_blanks();

        if (*txtpos != '=')
            goto qwhat;
        txtpos++;
        ignore_blanks();
        expression_error = 0;
        value = expression();
        if(expression_error)
            goto qwhat;
        // Check that we are at the end of the statement
        if(*txtpos != NL && *txtpos != ':')
            goto qwhat;
        *var = value;
    }
    goto run_next_statement;
poke:
    // Work out where to put it
    expression_error = 0;
    expression();
    if(expression_error)
        goto qwhat;

    // check for a comma
    ignore_blanks();
    if (*txtpos != ',')
        goto qwhat;
    txtpos++;
    ignore_blanks();

    // Now get the value to assign
    expression_error = 0;
    expression();
    if(expression_error)
        goto qwhat;
    // Check that we are at the end of the statement
    if(*txtpos != NL && *txtpos != ':')
        goto qwhat;
    goto run_next_statement;

list:
    linenum = testnum(); // Retuns 0 if no line found.

    // Should be EOL
    if(txtpos[0] != NL)
        goto qwhat;

    // Find the line
    list_line = findline();
    while(list_line != program_end)
        printline();
    goto warmstart;

print:
    // If we have an empty list then just put out a NL
    if(*txtpos == ':' ) {
        line_terminator();
        txtpos++;
        goto run_next_statement;
    }
    if(*txtpos == NL) {
        goto execnextline;
    }

    while(1) {
        ignore_blanks();
        if(print_quoted_string()) {
            ;
        } else if(*txtpos == '"' || *txtpos == '\'')
            goto qwhat;
        else {
            short int e;
            expression_error = 0;
            e = expression();
            if(expression_error)
                goto qwhat;
            printnum(e);
        }

        // At this point we have three options, a comma or a new line
        if(*txtpos == ',')
            txtpos++;	// Skip the comma and move onto the next
        else if(txtpos[0] == ';' && (txtpos[1] == NL || txtpos[1] == ':')) {
            txtpos++; // This has to be the end of the print - no newline
            break;
        } else if(*txtpos == NL || *txtpos == ':') {
            line_terminator();	// The end of the print statement
            break;
        } else
            goto qwhat;
    }
    goto run_next_statement;

mem:
    // memory free
    printnum(variables_begin-program_end);
    printmsg(memorymsg);
    goto run_next_statement;

    /*************************************************/
rseed: {
        short int value;

        //Get the pin number
        expression_error = 0;
        value = expression();
        if(expression_error)
            goto qwhat;

        srand( value );
        goto run_next_statement;
    }
}

/***************************************************************************/
static void line_terminator(void) {
    putchar(NL);
    putchar(CR);
}

/***********************************************************/
static unsigned char breakcheck(void) {
    if (kbhit()) {
        return getchar() == CTRLC;
    }
    return 0;
}
/***********************************************************/
static int inchar() {
    int got = getchar();
    // translation for desktop systems
    if( got == LF ) got = CR;
    return got;
}
