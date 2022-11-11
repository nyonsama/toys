const host =
  process.env.NODE_ENV === "development" ? "http://127.0.0.1:5677" : "";

// /scals.json对应后端的build/scals.json
export const learningBoardDataUrl = host + "/scals.json";
export const fileUploadUrl = host + "/api/upload";
export const md5TableDataUrl = host + "/graph3_nodes.json";
export const currentFileInfoUrl = host + "/api/fileinfo";
export const geneGraphUrl = [
  host + "/graph2.json",
  host + "/graph2.json",
  host + "/graph2.json",
];
export const featureDetailUrl = {
  gene: host + "/train.json",
  vector: host + "/APT_Vector.json",
};
export const historyUrl = host + "/api/history";
export const saveUrl = host + "/api/save";
export const aptCountUrl = host + "/aptcount.json";
export const analyzeResultUrl = host + "/result.json";

export const pieDataUrl = host + "fix me";

export interface PieData {
  first: { name: string; value: number }[];
  second: { name: string; value: number }[];
}

export type LearningBoardData = {
  train_acc_xy: [number, number][];
  train_loss_xy: [number, number][];
  val_acc_xy: [number, number][];
  val_loss_xy: [number, number][];
};

export interface Md5TableData {
  nodes: [
    {
      id: number;
      label: string;
      title: string;
      size: number;
      name: string;
      ttype: string;
      mdd: string;
    }
  ];
}

export interface FileInfo {
  filename?: string;
  sha256?: string;
  md5?: string;
  filetype?: string;
  filesize?: string;
  date?: string;
  res?: string;
}

export interface HistoryData {
  md5: string;
  apt: string;
  res: string;
  filetype: string;
  filesize: string;
  date: string;
}

export interface FeatureDetail {
  gene: {
    gene: string[];
    md5: string;
    family: string;
  }[];
  vector: {
    md5: string;
    type: string;
    family: string;
    vector: number[];
  }[];
}

export const fetchMd5TableData = async () => {
  const res = await fetch(md5TableDataUrl);
  return res.json() as Promise<Md5TableData>;
};

export const fetchCurrentFileInfo = async () => {
  const res = await fetch(currentFileInfoUrl);
  return res.json() as Promise<FileInfo>;
};

export const fetchLearningBoardData = async () => {
  const res = await fetch(learningBoardDataUrl);
  return res.json() as Promise<LearningBoardData>;
};

export const fetchFeatureDetail = async (): Promise<FeatureDetail> => {
  const gene = fetch(featureDetailUrl.gene).then((res) => res.json());

  const vector = fetch(featureDetailUrl.vector).then((res) => res.json());

  return Promise.all([gene, vector]).then(([gene, vector]) => ({
    gene,
    vector,
  }));
};

export const fetchHistory = async () => {
  const res = await fetch(historyUrl);
  const history: Array<any> = await res.json();

  history.forEach((e) => {
    e.apt = e.apt === "true" ? "是" : "否";
  });
  return history as HistoryData[];
};

export const saveCurrentFileInfo = async () => {
  return fetch(saveUrl);
};

export const fetchAptCount = async () => {
  const res = await fetch(aptCountUrl);
  return res.json();
};

export interface AnalyzeResult {
  detect: string;
  classify: any;
}
export const fetchAnalyzeResult = async () => {
  const res = await fetch(analyzeResultUrl);
  return res.json() as Promise<AnalyzeResult>;
};
