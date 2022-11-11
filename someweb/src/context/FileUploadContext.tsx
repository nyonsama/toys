import fileSize from "filesize";
import { createContext, ReactNode, useContext, useRef, useState } from "react";
import { Button, Modal, ProgressBar } from "react-bootstrap";
import { useNavigate } from "react-router-dom";
import axios from "axios";
import { fileUploadUrl } from "../lib/api";

export const FileUploadContext = createContext(() => {});

export const FileUploadProvider = (props: { children: ReactNode }) => {
  // 上传完成之后重定向到这里
  const redirectUrl = "/summary";

  const fileInputRef = useRef<HTMLInputElement>(null);

  const navigate = useNavigate();

  const [showModal, setShowModal] = useState(false);
  const [fileName, setFileName] = useState("");
  const [uploadProgress, setUploadProgress] = useState({ loaded: 0, total: 0 });

  const [showFailModal, setShowFailModal] = useState(false);
  const [failInfo, setFailInfo] = useState<any>({});

  const controller = new AbortController();

  const onFileUpload = () => {
    if (
      fileInputRef.current == null ||
      fileInputRef.current.files == null ||
      fileInputRef.current.files.length === 0
    ) {
      return;
    }

    const file = fileInputRef.current.files[0];
    const form = new FormData();
    form.append("file", file);
    setFileName(file.name);

    setShowModal(true);
    axios
      .post(fileUploadUrl, form, {
        headers: {
          "Content-Type": "multipart/form-data",
        },
        onUploadProgress: (e) => {
          setUploadProgress({ loaded: e.loaded, total: e.total });
        },
        signal: controller.signal,
      })
      .then((res) => {
        setShowModal(false);
        if (res.data.status !== 1) {
          setFailInfo(res.data);
          setShowFailModal(true);
        } else {
          if (window.location.pathname === "/summary") {
            window.location.reload();
          } else {
            navigate(redirectUrl);
          }
        }
      })
      .catch((err) => {
        console.log(err);
      });
    fileInputRef.current.value = "";
  };

  const onModalClose = () => {
    controller.abort();
    setShowModal(false);
    setUploadProgress({ loaded: 0, total: 0 });
  };

  const onFailModalClose = () => {
    setShowFailModal(false);
  };

  return (
    <FileUploadContext.Provider
      value={() => {
        fileInputRef.current?.click();
      }}
    >
      {props.children}
      <input
        name="file"
        type="file"
        ref={fileInputRef}
        onInput={(e) => {
          onFileUpload();
        }}
        style={{ display: "none" }}
      />
      <Modal
        show={showModal}
        onHide={onModalClose}
        backdrop="static"
        keyboard={false}
        centered
      >
        <Modal.Header closeButton>
          <Modal.Title>正在上传</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <ProgressBar
            animated
            now={
              uploadProgress.total === 0
                ? 0
                : (uploadProgress.loaded / uploadProgress.total) * 100
            }
          />
          <div className="d-flex justify-content-between">
            <p className="mb-0 mt-2">{fileName}</p>
            <p className="mb-0 mt-2">
              {fileSize(uploadProgress.loaded, { base: 2 })}/
              {fileSize(uploadProgress.total, { base: 2 })}
            </p>
          </div>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="outline-dark" onClick={onModalClose}>
            取消
          </Button>
        </Modal.Footer>
      </Modal>
      <Modal
        show={showFailModal}
        onHide={onFailModalClose}
        backdrop="static"
        keyboard={false}
        centered
      >
        <Modal.Header closeButton>
          <Modal.Title>错误</Modal.Title>
        </Modal.Header>
        <Modal.Body>{failInfo.info}</Modal.Body>
        <Modal.Footer>
          <Button variant="outline-dark" onClick={onFailModalClose}>
            确认
          </Button>
        </Modal.Footer>
      </Modal>
    </FileUploadContext.Provider>
  );
};

export const useFileUpload = () => useContext(FileUploadContext);
