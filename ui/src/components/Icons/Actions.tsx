import { FaSquare } from "react-icons/fa";
import { BsTriangleFill } from "react-icons/bs";
import { BaseIcon } from "./BaseIcon";

type ActionProps = {};

export const MajorAction = ({}: ActionProps) => {
  return (
    <BaseIcon size={22}>
      <FaSquare
        style={{
          color: "var(--gray-9)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorAction = ({}: ActionProps) => {
  return (
    <BaseIcon>
      <BsTriangleFill
        style={{
          color: "var(--gray-9)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorActionLight = ({}: ActionProps) => {
  return (
    <BaseIcon>
      <BsTriangleFill
        style={{
          color: "var(--gray-6)",
        }}
      />
    </BaseIcon>
  );
};
