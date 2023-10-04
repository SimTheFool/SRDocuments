import { FaSquare } from "react-icons/fa";
import { IoTriangleSharp } from "react-icons/io5";
import { BaseIcon } from "./BaseIcon";
import { FaSquareFull } from "react-icons/fa";

type ActionProps = { size?: number };

export const MajorAction = ({ size }: ActionProps) => {
  return (
    <BaseIcon size={size || 22}>
      <FaSquareFull
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorAction = ({ size }: ActionProps) => {
  return (
    <BaseIcon size={size || 20}>
      <IoTriangleSharp
        style={{
          color: "var(--gray-11)",
        }}
      />
    </BaseIcon>
  );
};

export const MinorActionLight = ({ size }: ActionProps) => {
  return (
    <BaseIcon size={size || 20}>
      <IoTriangleSharp
        style={{
          color: "var(--gray-6)",
        }}
      />
    </BaseIcon>
  );
};
