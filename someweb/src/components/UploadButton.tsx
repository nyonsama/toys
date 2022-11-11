import styled from "@emotion/styled";

const UploadButton = styled.div`
  width: 12rem;
  height: 4rem;
  border: 2px solid #12caeb;
  border-radius: 3rem;
  padding: 0 2rem;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 1.5rem;
  color: white;
  user-select: none;
  background-color: transparent;
  &:hover {
    /* background-color: #219bb163; */
    cursor: pointer;
    animation: light 0.2s ease;
    animation-fill-mode: both;
  }
  &:active {
    background-color: #13647262 !important;
  }
  @keyframes light {
    to {
      background-color: #219bb163;
    }
  }
`;

export default UploadButton;
