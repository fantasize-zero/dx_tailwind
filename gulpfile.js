const gulp = require("gulp");
const postcss = require("gulp-postcss");
const sass = require("gulp-sass")(require("sass"));
const rename = require("gulp-rename");

const postcssImport = require("postcss-import");
const postcssNested = require("postcss-nested");
const tailwindPostcss = require("@tailwindcss/postcss");

// 配置 autoprefixer
const autoprefixer = require("autoprefixer")({
  overrideBrowserslist: [
    "Android 4.1",
    "iOS 7.1",
    "Chrome > 31",
    "ff > 31",
    "ie >= 8",
  ],
});
// 配置 cssnano
const cssnano = require("cssnano")({
  preset: "default",
  autoprefixer: true, // 必须开启,不然tailwindcss的@apply渲染出来的样式不会添加浏览器前缀
});

// PostCSS 公共处理
function postcssPipeline() {
  return postcss([
    tailwindPostcss,
    postcssImport,
    postcssNested,
    autoprefixer,
    cssnano,
  ]).on("error", function (err) {
    console.error(err.toString());
    this.emit("end");
  });
}

function buildCSS() {
  return gulp
    .src("styles/**/*.css")
    .pipe(postcssPipeline())
    .pipe(rename({ suffix: ".min" }))
    .pipe(gulp.dest("assets"));
}

function buildSCSS() {
  return gulp
    .src("styles/**/*.scss")
    .pipe(sass().on("error", sass.logError))
    .pipe(postcssPipeline())
    .pipe(rename({ suffix: ".min" }))
    .pipe(gulp.dest("assets"));
}

// 组合处理 CSS 和 SCSS 文件的任务
gulp.task("build:css", gulp.parallel(buildCSS, buildSCSS));
