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

// 生成Control文件
func GenCtl(config Config) {
	//fmt.Println(config)

	ctlName := config.Table + "_" + time.Now().Format("20060102150405") + ".ctl"
	dataFileName := config.File + "_" + time.Now().Format("20060102150405") + ".tmp"

	fieldsOrder := config.Table

	if len(config.TableFields) != 0 && len(config.FileFields) != 0 {
		if len(config.TableFields) != len(config.FileFields) {
			fmt.Println("设置的数据文件字段数目与表字段列表数目不一致")
			os.Exit(1)
		}
		fieldsOrder = fieldsOrder + "(" + strings.Join(config.TableFields, ",") + ")"
	}

	fileFields := config.FileFields
	var ctl string

	if len(fileFields) == 0 {
		ctl = fmt.Sprintf(
			"file '%v' delimiter '%v' %v;\ninsert into %v;",
			dataFileName, config.Delimiter, config.Numcols, fieldsOrder)
	} else {
		ctl = fmt.Sprintf(
			"file '%v' delimiter '%v' %v;\ninsert into %v values(",
			dataFileName, config.Delimiter, config.Numcols, fieldsOrder)

		i := 1
		//var cols []int
		for _, col := range fileFields {
			//cols = append(cols, col)
			if i != 1 {
				ctl = ctl + ","
			}
			ctl = ctl + fmt.Sprintf("f%02s", strconv.Itoa(col))
			i += 1
		}

		ctl += ");"

	}

	f1, err := os.Create(ctlName)
	if err != nil {
		fmt.Println(err)
	}
	defer f1.Close()
	_, err = f1.WriteString(ctl)
	if err != nil {
		fmt.Println(err)
	}

	f2, err := os.Create("tmp_ctlname")
	if err != nil {
		fmt.Println(err)
	}
	defer f2.Close()
	_, err = f2.WriteString(ctlName)
	if err != nil {
		fmt.Println(err)
	}

	f3, err := os.Create("tmp_datafilename")
	if err != nil {
		fmt.Println(err)
	}
	defer f3.Close()
	_, err = f3.WriteString(dataFileName)
	if err != nil {
		fmt.Println(err)
	}
}

func ReadLines(path string, config map[int]string, delimiter string, enclose string, hasTitle int) error {
	file, err := os.Open(path)
	if err != nil {
		return err
	}
	defer file.Close()

	//var lines []string
	scanner := bufio.NewScanner(file)

	//var row1 []string

	for scanner.Scan() {
		if hasTitle == 1 {
			hasTitle = 0
			continue
		}
		row := DealRow(scanner.Text(), delimiter, enclose)

		for key, value := range config {
			if len(config[key]) != 0 {
				key -= 1
				row[key] = FmtFunc(row[key], value)
			}

		}
		str := strings.Join(row, delimiter) + delimiter
		fmt.Println(str)
	}
	return scanner.Err()
}

func DealEnClosed(str string, enclose string) int {
	/**
		1、不包含"返回 0
	 	2、首尾包含"", 返回 2
	 	3、仅左边包含, 返回 -1
	 	4、仅右边包含, 返回 1
	*/

	//encloseByte := enclose[0]

	lenStr := len(str)
	lenEnclose := len(enclose)

	if !strings.Contains(str, enclose) {
		return 0
	} else if str[0:lenEnclose] == enclose {

		if str[lenStr-lenEnclose:lenStr] == enclose {
			return 2
		} else {
			return -1
		}
	} else if str[lenStr-lenEnclose:lenStr] == enclose {
		return 1
	}
	return 0
}

func DealRow(str string, delimiter string, enclose string) []string {
	row1 := strings.Split(str, delimiter)
	var row []string

	flag := true
	var start int
	var end int

	//lenStr := len(str)
	lenEnclose := len(enclose)

	for i, v := range row1 {
		if flag {
			if DealEnClosed(v, enclose) == 0 {
				row = append(row, v)
			}
			if DealEnClosed(v, enclose) == 2 {
				row = append(row, v[lenEnclose:len(v)-lenEnclose])
			}
			if DealEnClosed(v, enclose) == -1 {
				flag = false
				start = i
			}
		} else {
			if DealEnClosed(v, enclose) == 1 {
				flag = true
				end = i
				tmp := strings.Join(row1[start:end+1], "\\"+delimiter)
				row = append(row, tmp[lenEnclose:len(tmp)-lenEnclose])
			}
		}
	}
	return row
}



func deal1() {
	return 0;
}

