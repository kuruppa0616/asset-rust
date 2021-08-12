const doPost = (e) => {
    const ss = SpreadsheetApp.getActive();
    const sheet = ss.getSheetByName("SbiAsset");
    const contents  =JSON.parse(e.postData.contents);
    appendOrUpdateData(sheet, contents);
  
  }
  
  const appendOrUpdateData = (sheet, contents) => {
    const date = new Date(contents.date);
    const total = contents.total;
    const profit = contents.profit;
    const profit_percent = contents.profit_percent;
  
    const idx = findExistRow(sheet, date);
    if(idx == -1){
      sheet.appendRow([date, total, profit, profit_percent]);
    }else{
      sheet.getRange(idx, 1, 1, Object.keys(contents).length).setValues([[date, total, profit, profit_percent]]);
    }
  }
  
  
  // 日付にマッチする列番号を探す
  const findExistRow = (sheet, searchDate) => {
    const values = sheet.getDataRange().getValues();
  
    for (var i = values.length - 1; i > 0; i--) {
      var targetDate = new Date(values[i][0]);
      if (targetDate.getTime() === searchDate.getTime()) {
        return i + 1;
      }
    }
    return -1;
  } 
  
  