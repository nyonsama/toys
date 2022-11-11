/** @jsxImportSource @emotion/react */
import { css } from "@emotion/react";

import { AiOutlineTeam } from "react-icons/ai";
import _ from "lodash";

const AptCard = (props: {
  name: string;
  link: string;
  description: string;
}) => {
  return (
    <div
      css={css`
        display: flex;
        & a {
          text-decoration: none;
          color: white;
        }
        & hr {
          margin: 0.5rem 0;
        }
        & p {
          margin: unset;
        }
      `}
    >
      <AiOutlineTeam
        css={css`
          width: 6rem;
          height: 6rem;
          margin-right: 1rem;
          flex-shrink: 0;
          color: #ffffffa0;
        `}
      />
      <div>
        <a href={props.link}>{props.name}</a>
        <hr />
        <p className="fw-light">{props.description}</p>
      </div>
    </div>
  );
};

const aptList = [
  {
    name: "MuddyWater",
    link: "https://www.freebuf.com/articles/web/165061.html",
    description: "MuddyWater又名T-APT-14，其主要攻击目标在伊朗，于2014年暴露。",
  },
  {
    name: "海莲花",
    link: "https://blogs.360.cn/post/oceanlotus-apt.html",
    description:
      "其又名APT32，是一种长期针对中国进行攻击的APT组织，产生自2012年",
  },
  {
    name: "Donot",
    link: "https://www.freebuf.com/column/178841.html",
    description:
      "又名APT—C—35，其攻击范围非常广泛，使用的语言也五花八门，疑似是受南亚某国支持",
  },
  {
    name: "毒云藤",
    link: "https://blogs.360.cn/post/APT_C_01.html",
    description:
      "又名APT—C—1，其和海莲花之间有部分相关联的地方，产生于2007年，主要针对中国攻击",
  },
  {
    name: "Hangover",
    link: "https://blogs.360.cn/post/%E6%91%A9%E8%AF%83%E8%8D%89%E7%BB%84%E7%BB%87.html",
    description:
      "又名APT-C-09，是一个来自南亚的apt组织，自2013年曝光，主要针对中国及巴基斯坦。",
  },
  {
    name: "双尾蝎",
    link: "https://www.freebuf.com/articles/system/129223.html",
    description: "又名APT-C-23，主要攻击巴基斯坦等中东地区，自2016年曝光，",
  },
  {
    name: "黑凤梨",
    link: "https://www.freebuf.com/column/159865.html",
    description:
      "又名T-APT-03，是一个长期活动于亚洲地区的APT组织，最早可以追溯至2011年",
  },
  {
    name: "蔓灵花",
    link: "https://blogs.360.cn/post/analysis_of_APT_C_08.html",
    description:
      "又名APT-C-08，是一个来自南亚的APT组织，主要针对中国，巴基斯坦，暴露于2016年",
  },
  {
    name: "MalluCyberSoldiers",
    link: "http://it.rising.com.cn/dongtai/19587.html",
    description:
      "一个非常高调的APT组织，甚至可以直接在facebook上找到他们，其仍然来自南亚，主要针对巴基斯坦。",
  },
  {
    name: "盲眼鹰",
    link: "https://www.freebuf.com/column/196112.html",
    description:
      "又名APT-C-36，疑似来自南美，被发现于2018年，主要针对哥伦比亚政府。",
  },
];

const AptFamilySection = () => {
  const cards = _.chunk(aptList, 2).map((e, i) => {
    const row = e.map((ee, ii) => (
      <div className="col-sm-6" key={`${i}${ii}`}>
        <AptCard {...ee} />
      </div>
    ));
    return (
      <div className="row" key={i}>
        {row}
      </div>
    );
  });
  return (
    <div
      className="container d-flex flex-column justify-content-center"
      css={css`
        & > div {
          margin-bottom: 2rem;
        }
        & > div:last-of-type {
          margin-bottom: unset;
        }
        & > div > div:nth-of-type(1) {
          padding-right: 1rem;
        }
        & > div > div:nth-of-type(2) {
          padding-left: 1rem;
        }
      `}
    >
      <span className="fs-3">常见的APT家族</span>
      <hr />
      {cards}
    </div>
  );
};

export default AptFamilySection;
