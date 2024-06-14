.PHONY: all
all: TKY2JGD.in touhokutaiheiyouoki2011.in

%.in: %.par
	cargo run --package conv < $*.par > $*.in 2> $*.in.log

%.par: %.zip
	unzip -p $*.zip > $@

TKY2JGD.zip:
# 座標変換パラメータファイル Ver.2.1.2 2003/10/07公開
	curl --output $@ https://www.gsi.go.jp/common/000185226.zip

touhokutaiheiyouoki2011.zip:
# 座標補正パラメータファイル 平成23年(2011年)東北地方太平洋沖地震 Ver.4.0.0 2017/12/05公開
	curl --output $@ https://www.gsi.go.jp/common/000184959.zip
