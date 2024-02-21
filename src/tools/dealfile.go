package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
	"time"
)

type Config struct {
	File        string         `json:"file"`
	Delimiter   string         `json:"delimiter"`
	Numcols     int            `json:"numcols"`
	Table       string         `json:"table"`
	DateFormat  map[int]string `json:"dateformat"`
	FileFields  []int          `json:"filefields"`
	TableFields []string       `json:"tablefields"`
	Enclose     string         `json:"enclose"`
	HasTitle    int            `json:"hastitle"`
}

func main() {
	var config Config

	configPath := os.Args[1]
	flag := os.Args[2]

	configFile, err := os.Open(configPath)
	if err != nil {
		fmt.Printf("配置文件打开失败: %v\n", err)
		return
	}
	defer configFile.Close()

	configData, err := ioutil.ReadAll(configFile)
	if err != nil {
		fmt.Printf("配置读取失败: %v\n", err)
		return
	}

	json.Unmarshal(configData, &config)
	//fmt.Println(config.Fields[4])
	switch flag {
	case "0":
		GenCtl(config)
	case "1":
		ReadLines(config.File, config.DateFormat, config.Delimiter, config.Enclose, config.HasTitle)
	}

}

func FmtFunc(inTime string, fmtStr string) string {

	fmtStr = strings.ToLower(fmtStr)

	fmtStr = strings.ReplaceAll(fmtStr, "yyyy", "2006")
	fmtStr = strings.ReplaceAll(fmtStr, "mm", "01")
	fmtStr = strings.ReplaceAll(fmtStr, "dd", "02")
	fmtStr = strings.ReplaceAll(fmtStr, "hh", "15")
	fmtStr = strings.ReplaceAll(fmtStr, "mi", "04")
	fmtStr = strings.ReplaceAll(fmtStr, "ss", "05")

	//fmt.Println(fmtStr)

	outTime, err := time.Parse(fmtStr, inTime)
	if err != nil {
		return inTime
	}

	return outTime.Format("2006-01-02 15:04:05")
}
