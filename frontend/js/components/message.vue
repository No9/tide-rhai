<template>
    <div style="overflow: hidden;"></div>
</template>

<script>
import CodeMirror from "codemirror";
import "codemirror/lib/codemirror.css";
import "codemirror/addon/comment/comment";
import "codemirror/addon/display/rulers";
import "codemirror/addon/edit/closebrackets";
import "codemirror/addon/edit/matchbrackets";
import "codemirror/addon/fold/brace-fold";
import "codemirror/addon/fold/foldgutter";
import "codemirror/addon/fold/foldgutter.css";
import "codemirror/addon/search/match-highlighter";
import "codemirror/addon/selection/active-line";

function initEditor(vm) {
    const editor = CodeMirror(vm.$el, {
    mode: {name: "javascript", json: true},
    lineNumbers: true,
    lineWrapping: true,
    extraKeys: {"Ctrl-Q": function(cm){ cm.foldCode(cm.getCursor()); }},
    foldGutter: true,
    gutters: ["CodeMirror-linenumbers", "CodeMirror-foldgutter"],
    foldOptions: {
      widget: (from, to) => {
        var count = undefined;

        // Get open / close token
        var startToken = '{', endToken = '}';        
        var prevLine = window.editor_json.getLine(from.line);
        if (prevLine.lastIndexOf('[') > prevLine.lastIndexOf('{')) {
          startToken = '[', endToken = ']';
        }

        // Get json content
        var internal = window.editor_json.getRange(from, to);
        var toParse = startToken + internal + endToken;

        // Get key count
        try {
          var parsed = JSON.parse(toParse);
          count = Object.keys(parsed).length;
        } catch(e) { }        

        return count ? `\u21A4${count}\u21A6` : '\u2194';
      }
    }
  });

    editor.on("change", (editor, changes) => {
        vm.change(editor, changes);
    });

    return editor;
}

export default {
    methods: {
        change(editor, changes) {
            this.$emit("change", editor, changes);
        },
        requestRun() {
            this.$emit("requestRun", this.$_cm);
        },
        getEditor() {
            return this.$_cm;
        },
    },
    mounted() {
        this.$_cm = initEditor(this);
    },
};
</script>
