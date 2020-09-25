%{
    #include<stdio.h>
    #include <stdlib.h>
    #include <string.h>
    
    // similar to yylval, but not an enum, as we alway require strings
    char *tok_val = NULL;
    char *reference = NULL;
    char *filename = NULL;
    int count = 0;

    /* FIXED CONSTANTS*/
    /* Value of these must always be same,as these are defined independently
        in the Rust detector, and used.
        Maybe a solution would be to FFI bind these as well,
        but that will require extra setup fns as well as uneccessary unsafe {}
     */
    #define EOL 256
    
    /* TYPES OF TOKENS */
    /* first must alway be last constant value defined above +1 */
    enum Tokens{
        // For generic tokens such as operators and such
        GENERIC = EOL + 1,
        // Any keyword !? SHOULD REPLACE BY INDIVIDUAL?
        KEYWORD ,
        // A function defination call
        FUNCTION,
        // A number, float or int
        NUMBER ,
        // An Identifier
        IDENTIFIER,
        // For strings
        STRING,
        // for comments, !? probably should just ignore comments?
        COMMENT,
    };
    
    
    // funtions which will set-reset value of tok_val
    void set_val(int offset);
    void reset_val(void);
    void yyerror(char* msg);
    void tty_open();
%}

%option nodefault noyywrap yylineno

%x COMMENT
%x REF

%%

[ \n\t]                                     {/* no perticular action, as yylineno auto-incremetns*/}

"-----> START SAMPLE ref"[ ]*":"[ ]*       {BEGIN REF;}
<REF>[^ \n\t]*                   {if(reference)free(reference);reference = strdup(yytext);yylineno=0;BEGIN INITIAL;}
<REF>[ \n\t]*                         {yylineno=0;BEGIN INITIAL;}
<REF><<EOF>>                       {yyerror("EOF After reference line.");yyterminate();}


"/*"                                {BEGIN COMMENT;}
<COMMENT>"*/"                       {BEGIN INITIAL;}
<COMMENT>([^*]|\n)+|.              {set_val(0);return COMMENT;}
<COMMENT><<EOF>>                    {yyerror("Unterminated multiline comment.");yyterminate();}

"//".*$                         {set_val(2);return COMMENT;}

"{"|"}"                             {return EOL;}
"("|")"|"["|"]"                    {} /*{set_val(0);return GENERIC;}*/
"+"|"-"|"/"|"*"|"%"|"="                     {}/*{set_val(0);return GENERIC;}*/

0[0-7]*|[0-9]*|0[Xx][0-9a-fA-f]+      { set_val(0);return NUMBER; }

([0-9]*\.[0-9]+|[0-9]*\.[0-9]+([Ee][-+]?[0-9]+)?)  { set_val(0);return NUMBER; }

"auto"|"break"|"case"|"char"|"const"|"continue"|"default"|"do"              {set_val(0);return KEYWORD;}
"double"|"else"|"enum"|"extern"|"float"|"for"|"goto"|"if"                   {set_val(0);return KEYWORD;}
"int"|"long"|"register"|"return"|"short"|"signed"|"sizeof"|"static"         {set_val(0);return KEYWORD;}
"struct"|"switch"|"typedef"|"union"|"unsigned"|"void"|"volatile"|"while"    {set_val(0);return KEYWORD;}

;                                       {return EOL;}

[_a-zA-Z][_a-zA-Z0-9]*           {set_val(0);return IDENTIFIER;}

\"(\\.|[^\"\n])*\"                      {set_val(0);return STRING;}
\"(\\.|[^\"\n])*$                       {yyerror("unterminated string"); yyterminate();}

.                                   {fprintf(stderr,"This chracter was not anticipated %s \n",yytext);}

<<EOF>>                             {return EOF;}
%%

void yyerror(char * msg){
    fprintf(stderr,"ERROR on line %d : %s",yylineno,msg);
}

void set_val(int offset){
    
    if(tok_val != NULL) free(tok_val);
    
    tok_val = calloc(strlen(yytext-offset),sizeof(char));
    
    strcpy(tok_val,yytext+offset);
    
    
}

void reset_val(void){
    if (tok_val == NULL) return;

    free(tok_val);
    tok_val = NULL;
    return;
}

void tty_open(){
   if(yyin)fclose(yyin);
   char buf[50];
    
    if( filename == NULL){
        FILE* in = fopen("/dev/tty","r");
        printf("give file : ");
        fscanf(in,"%s",buf);
        yyin = fopen(buf,"r");
        fclose(in);
    }else{
        yyin = fopen(filename,"r");
    }
    
    
    yylineno = 1;
    
    return;
}
