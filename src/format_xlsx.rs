use std::{collections::{HashMap, hash_map::Entry}, fs::{read, rename}, io::Write};

use chrono::Local;
use zip::{ZipWriter, write::FileOptions, CompressionMethod};

use crate::{param::PriceVolume, price::{Show, PriceItem, ValueType}};

pub struct FormatXLSX { }

impl FormatXLSX {

    fn escape_xml(val: &str) -> String {
        let mut new = val.replace("&", "&amp;");
        new = new.replace("\"", "&quot;");
        new = new.replace("'", "&apos;");
        new = new.replace("<", "&lt;");
        new = new.replace(">", "&gt;");
        new
    }

    fn get_col_xlsx(v: &Vec<char>, col: usize) -> Option<String> {
        if col > 702 { return None; }

        if col < 26 {
            return Some(v[col].to_string());
        }

        let x: usize = col / 26 - 1;
        let f = v[x];
        let x: usize = col - (x + 1) * 26;
        let s = v[x];
        Some(format!("{}{}", f, s))
    }

    pub fn make(items: &HashMap<u32, PriceItem>, filename: &str, volume: &PriceVolume, rozn: bool, r3: bool, ean: bool) -> Option<Vec<u8>> {
        let mut show = Show::new();
        let mut count_shared: u32 = 0;
        let mut dict: HashMap<&str, usize> = HashMap::with_capacity(20 * (items.len()+1));
        let alfa: Vec<char> = ('A'..='Z').into_iter().collect();
        let mut col: usize = 0;
        match volume {
            PriceVolume::Local => {
                for item in &mut show.list {
                    if item.local || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = match FormatXLSX::get_col_xlsx(&alfa, col) {
                            Some(i) => Some(i),
                            None => return None,
                        };
                        col += 1;
                    }
                }
            },
            PriceVolume::Full => {
                for item in &mut show.list {
                    if item.full || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = match FormatXLSX::get_col_xlsx(&alfa, col) {
                            Some(i) => Some(i),
                            None => return None,
                        };
                        col += 1;
                    }
                }
            },
            PriceVolume::Short => {
                for item in &mut show.list {
                    if item.short || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = match FormatXLSX::get_col_xlsx(&alfa, col) {
                            Some(i) => Some(i),
                            None => return None,
                        };
                        col += 1;
                    }
                }
            },
            PriceVolume::FullUAH => {
                for item in &mut show.list {
                    if item.full_uah || (item.rozn && rozn) || (item.r3 && r3) || (item.ean && ean) {
                        item.index = match FormatXLSX::get_col_xlsx(&alfa, col) {
                            Some(i) => Some(i),
                            None => return None,
                        };
                        col += 1;
                    }
                }

            },
        }

        let col = match FormatXLSX::get_col_xlsx(&alfa, col - 1) {
            Some(i) => i,
            None => return None,
        };
        let tmp = format!("{}.tmp", filename);
        let path = std::path::Path::new(&tmp);
        let zipfile = match std::fs::File::create(&path) {
            Ok(zip) => zip,
            Err(_) => return None,
        };
        let options = FileOptions::default().compression_level(Some(3)).compression_method(CompressionMethod::Deflated);

        let mut zip = ZipWriter::new(zipfile);
        {
            if let Err(_) = zip.start_file("[Content_Types].xml", options) {
                return None;
            };
            let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"><Override PartName=\"/_rels/.rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Override PartName=\"/xl/_rels/workbook.xml.rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/><Override PartName=\"/xl/worksheets/sheet1.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\"/><Override PartName=\"/xl/sharedStrings.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml\"/><Override PartName=\"/xl/workbook.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\"/><Override PartName=\"/xl/styles.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\"/><Override PartName=\"/docProps/app.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.extended-properties+xml\"/><Override PartName=\"/docProps/core.xml\" ContentType=\"application/vnd.openxmlformats-package.core-properties+xml\"/></Types>";
            if let Err(_) = zip.write_all(data.as_bytes()) {
                return None;
            }
        }
        {
            if let Err(_) = zip.add_directory("_rels", options) {
                return None;
            };
            {
                let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument\" Target=\"xl/workbook.xml\"/><Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties\" Target=\"docProps/core.xml\"/><Relationship Id=\"rId3\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties\" Target=\"docProps/app.xml\"/></Relationships>";
                if let Err(_) = zip.start_file("_rels/.rels", options) {
                    return None;
                };
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
        }
        {
            if let Err(_) = zip.add_directory("docProps", options) {
                return None;
            };
            {
                let data = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<Properties xmlns=\"http://schemas.openxmlformats.org/officeDocument/2006/extended-properties\" xmlns:vt=\"http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes\"><TotalTime>0</TotalTime></Properties>";
                if let Err(_) = zip.start_file("docProps/app.xml", options) {
                    return None;
                };
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
            {
                let dt = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
                let data = format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<cp:coreProperties xmlns:cp=\"http://schemas.openxmlformats.org/package/2006/metadata/core-properties\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:dcmitype=\"http://purl.org/dc/dcmitype/\" xmlns:dcterms=\"http://purl.org/dc/terms/\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"><dcterms:created xsi:type=\"dcterms:W3CDTF\">{}.00Z</dcterms:created><dc:creator>Brain</dc:creator><cp:revision>0</cp:revision></cp:coreProperties>", dt);
                if let Err(_) = zip.start_file("docProps/core.xml", options) {
                    return None;
                };
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
        }
        {
            if let Err(_) = zip.add_directory("xl", options) {
                return None;
            };
            {
                if let Err(_) = zip.add_directory("xl/_rels", options) {
                    return None;
                };
                {
                    let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" Target=\"styles.xml\"/><Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet\" Target=\"worksheets/sheet1.xml\"/><Relationship Id=\"rId3\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings\" Target=\"sharedStrings.xml\"/></Relationships>";
                    if let Err(_) = zip.start_file("xl/_rels/workbook.xml.rels", options) {
                        return None;
                    };
                    if let Err(_) = zip.write_all(data.as_bytes()) {
                        return None;
                    }
                }
            }
            if let Err(_) = zip.add_directory("xl/worksheets", options) {
                return None;
            };
            let mut list: Vec<&str> = Vec::with_capacity(dict.capacity());
            {
                if let Err(_) = zip.start_file("xl/worksheets/sheet1.xml", options) {
                    return None;
                };
                let data: &str = &format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<worksheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><sheetPr filterMode=\"false\"><pageSetUpPr fitToPage=\"false\"/></sheetPr><dimension ref=\"A1:{}{}\"/><sheetViews><sheetView colorId=\"64\" defaultGridColor=\"true\" rightToLeft=\"false\" showFormulas=\"false\" showGridLines=\"true\" showOutlineSymbols=\"true\" showRowColHeaders=\"true\" showZeros=\"true\" tabSelected=\"true\" topLeftCell=\"A1\" view=\"normal\" windowProtection=\"false\" workbookViewId=\"0\" zoomScale=\"100\" zoomScaleNormal=\"100\" zoomScalePageLayoutView=\"100\"><selection activeCell=\"A1\" activeCellId=\"0\" pane=\"topLeft\" sqref=\"A1\"/></sheetView></sheetViews><cols><col collapsed=\"false\" hidden=\"false\" max=\"1025\" min=\"1\" style=\"0\" width=\"11.5\"/></cols><sheetData>", col, items.len() + 1);
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
                let mut data = String::with_capacity(10000000);
                let mut index: usize = 0;
                let mut num: usize;
                let mut count: u32 = 1;
                data.push_str(&format!("<row collapsed=\"false\" customFormat=\"false\" customHeight=\"false\" hidden=\"false\" ht=\"12.1\" outlineLevel=\"0\" r=\"{}\">", count));
                for item in &mut show.list {
                    if let Some(ind) = &item.index {
                        count_shared += 1;
                        num = match dict.entry(&item.name) {
                            Entry::Occupied(o) => *o.get(),
                            Entry::Vacant(v) => {
                                v.insert(index);
                                index += 1;
                                list.push(&item.name);
                                index -1
                            },
                        };
                        data.push_str(&format!("<c r=\"{}{}\" s=\"0\" t=\"s\"><v>{}</v></c>", ind, count, num));
                    }
                }
                data.push_str("</row>");
                count += 1;
                for (_, price) in items {
                    data.push_str(&format!("<row collapsed=\"false\" customFormat=\"false\" customHeight=\"false\" hidden=\"false\" ht=\"12.1\" outlineLevel=\"0\" r=\"{}\">", count));
                    for item in &show.list {
                        if let Some(ind) = &item.index {
                            match (item.get)(price) {
                                ValueType::String(val) => {
                                    count_shared += 1;
                                    num = match dict.entry(val) {
                                        Entry::Occupied(o) => *o.get(),
                                        Entry::Vacant(v) => {
                                            v.insert(index);
                                            index += 1;
                                            list.push(val);
                                            index - 1
                                        },
                                    };
                                    data.push_str(&format!("<c r=\"{}{}\" s=\"0\" t=\"s\"><v>{}</v></c>", ind, count, num));
                                },
                                ValueType::Money(v) => data.push_str(&format!("<c r=\"{}{}\" s=\"1\" t=\"n\"><v>{:.2}</v></c>", ind, count, v)),
                                ValueType::Index(v) => data.push_str(&format!("<c r=\"{}{}\" s=\"3\" t=\"n\"><v>{}</v></c>", ind, count, v)),
                            }
                            if data.len() > 9900000 {
                                if let Err(_) = zip.write_all(data.as_bytes()) {
                                    return None;
                                }
                                data.clear();
                            }
                        }
                    }
                    data.push_str("</row>");
                    count += 1;
                }
                data.push_str("</sheetData><printOptions headings=\"false\" gridLines=\"false\" gridLinesSet=\"true\" horizontalCentered=\"false\" verticalCentered=\"false\"/><pageMargins left=\"0.5\" right=\"0.5\" top=\"1.0\" bottom=\"1.0\" header=\"0.5\" footer=\"0.5\"/><pageSetup blackAndWhite=\"false\" cellComments=\"none\" copies=\"1\" draft=\"false\" firstPageNumber=\"1\" fitToHeight=\"1\" fitToWidth=\"1\" horizontalDpi=\"300\" orientation=\"portrait\" pageOrder=\"downThenOver\" paperSize=\"1\" scale=\"100\" useFirstPageNumber=\"true\" usePrinterDefaults=\"false\" verticalDpi=\"300\"/><headerFooter differentFirst=\"false\" differentOddEven=\"false\"><oddHeader>&amp;C&amp;&quot;Times New Roman,Regular&quot;&amp;12&amp;A</oddHeader><oddFooter>&amp;C&amp;&quot;Times New Roman,Regular&quot;&amp;12Page &amp;P</oddFooter></headerFooter></worksheet>");
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
            {
                if let Err(_) = zip.start_file("xl/workbook.xml", options) {
                    return None;
                };
                let data = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<workbook xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\" xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"><fileVersion appName=\"Calc\"/><workbookPr backupFile=\"false\" showObjects=\"all\" date1904=\"false\"/><workbookProtection/><bookViews><workbookView activeTab=\"0\" firstSheet=\"0\" showHorizontalScroll=\"true\" showSheetTabs=\"true\" showVerticalScroll=\"true\" tabRatio=\"212\" windowHeight=\"8192\" windowWidth=\"16384\" xWindow=\"0\" yWindow=\"0\"/></bookViews><sheets><sheet name=\"price\" sheetId=\"1\" state=\"visible\" r:id=\"rId2\"/></sheets><calcPr iterateCount=\"100\" refMode=\"A1\" iterate=\"false\" iterateDelta=\"0.001\"/></workbook>";
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
            {
                if let Err(_) = zip.start_file("xl/styles.xml", options) {
                    return None;
                };
                let data = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<styleSheet xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\"><numFmts count=\"4\"><numFmt formatCode=\"@\" numFmtId=\"164\"/><numFmt formatCode=\"0.00\" numFmtId=\"167\"/><numFmt formatCode=\"YYYY/MM/DD\\ HH:MM:SS\" numFmtId=\"166\"/><numFmt formatCode=\"0\" numFmtId=\"167\"/></numFmts><fonts count=\"4\"><font><name val=\"Arial\"/><charset val=\"1\"/><family val=\"2\"/><sz val=\"10\"/></font><font><name val=\"Arial\"/><family val=\"0\"/><sz val=\"10\"/></font><font><name val=\"Arial\"/><family val=\"0\"/><sz val=\"10\"/></font><font><name val=\"Arial\"/><family val=\"0\"/><sz val=\"10\"/></font></fonts><fills count=\"2\"><fill><patternFill patternType=\"none\"/></fill><fill><patternFill patternType=\"gray125\"/></fill></fills><borders count=\"1\"><border diagonalDown=\"false\" diagonalUp=\"false\"><left/><right/><top/><bottom/><diagonal/></border></borders><cellStyleXfs count=\"20\"><xf applyAlignment=\"true\" applyBorder=\"true\" applyFont=\"true\" applyProtection=\"true\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"164\"><alignment horizontal=\"general\" indent=\"0\" shrinkToFit=\"false\" textRotation=\"0\" vertical=\"bottom\" wrapText=\"false\"/><protection hidden=\"false\" locked=\"true\"/></xf><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"2\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"2\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"43\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"41\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"44\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"42\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"true\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"1\" numFmtId=\"9\"/></cellStyleXfs><cellXfs count=\"4\"><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"false\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"164\" xfId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"false\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"165\" xfId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"false\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"166\" xfId=\"0\"/><xf applyAlignment=\"false\" applyBorder=\"false\" applyFont=\"false\" applyProtection=\"false\" borderId=\"0\" fillId=\"0\" fontId=\"0\" numFmtId=\"167\" xfId=\"0\"/></cellXfs><cellStyles count=\"6\"><cellStyle builtinId=\"0\" customBuiltin=\"false\" name=\"Normal\" xfId=\"0\"/><cellStyle builtinId=\"3\" customBuiltin=\"false\" name=\"Comma\" xfId=\"15\"/><cellStyle builtinId=\"6\" customBuiltin=\"false\" name=\"Comma [0]\" xfId=\"16\"/><cellStyle builtinId=\"4\" customBuiltin=\"false\" name=\"Currency\" xfId=\"17\"/><cellStyle builtinId=\"7\" customBuiltin=\"false\" name=\"Currency [0]\" xfId=\"18\"/><cellStyle builtinId=\"5\" customBuiltin=\"false\" name=\"Percent\" xfId=\"19\"/></cellStyles></styleSheet>";
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
            {
                if let Err(_) = zip.start_file("xl/sharedStrings.xml", options) {
                    return None;
                };
                let data: &str = &format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n<sst count=\"{}\" uniqueCount=\"{}\" xmlns=\"http://schemas.openxmlformats.org/spreadsheetml/2006/main\">", count_shared, dict.len());
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
                let mut data = String::with_capacity(10000000);
                for val in list {
                    data.push_str(&format!("<si><t>{}</t></si>", FormatXLSX::escape_xml(val)));
                    if data.len() > 9900000 {
                        if let Err(_) = zip.write_all(data.as_bytes()) {
                            return None;
                        }
                        data.clear();
                    }
                }
                data.push_str("</sst>");
                if let Err(_) = zip.write_all(data.as_bytes()) {
                    return None;
                }
            }
        }
        if let Err(_) = zip.finish() {
            return None;
        };
        drop(zip);
        if let Err(_) = rename(&tmp, filename) {
            return None;
        }
        let path = std::path::Path::new(filename);
        match read(path) {
            Ok(str) => Some(str),
            Err(_) => None,
        }
    }

}